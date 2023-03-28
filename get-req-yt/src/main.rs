use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(request::Error);
    }
}

fn main() -> Result <()> {
    let mut res = request::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status :{}", res.status());

    println!("Headers: \n{:#?}", res.headers());
    println!("Body:\n{}", body);
    Ok(())
}