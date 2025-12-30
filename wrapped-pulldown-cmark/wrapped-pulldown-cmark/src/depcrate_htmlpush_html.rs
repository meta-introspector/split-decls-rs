// Generated macro for push_html (function)
macro_rules! Depcrate_htmlpush_html {
() => {
// Module: crate::html
// Provides: {"push_html"}
// Dependencies: {}
# [doc = " Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and"] # [doc = " push it to a `String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use pulldown_cmark::{html, Parser};"] # [doc = ""] # [doc = " let markdown_str = r#\""] # [doc = " hello"] # [doc = " ====="] # [doc = ""] # [doc = " * alpha"] # [doc = " * beta"] # [doc = " \"#;"] # [doc = " let parser = Parser::new(markdown_str);"] # [doc = ""] # [doc = " let mut html_buf = String::new();"] # [doc = " html::push_html(&mut html_buf, parser);"] # [doc = ""] # [doc = " assert_eq!(html_buf, r#\"<h1>hello</h1>"] # [doc = " <ul>"] # [doc = " <li>alpha</li>"] # [doc = " <li>beta</li>"] # [doc = " </ul>"] # [doc = " \"#);"] # [doc = " ```"] pub fn push_html < 'a , I > (s : & mut String , iter : I) where I : Iterator < Item = Event < 'a > > , { write_html_fmt (s , iter) . unwrap () }
};
}
