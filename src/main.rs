use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate {
}


fn main() {
    let template = MainTemplate {};
    println!("{}", template.render().unwrap());
}
