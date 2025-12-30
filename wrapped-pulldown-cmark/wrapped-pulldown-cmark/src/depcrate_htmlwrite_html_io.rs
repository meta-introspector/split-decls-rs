// Generated macro for write_html_io (function)
macro_rules! Depcrate_htmlwrite_html_io {
() => {
// Module: crate::html
// Provides: {"write_html_io"}
// Dependencies: {}
# [doc = " Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and"] # [doc = " write it out to an I/O stream."] # [doc = ""] # [doc = " **Note**: using this function with an unbuffered writer like a file or socket"] # [doc = " will result in poor performance. Wrap these in a"] # [doc = " [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html) to"] # [doc = " prevent unnecessary slowdowns."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use pulldown_cmark::{html, Parser};"] # [doc = " use std::io::Cursor;"] # [doc = ""] # [doc = " let markdown_str = r#\""] # [doc = " hello"] # [doc = " ====="] # [doc = ""] # [doc = " * alpha"] # [doc = " * beta"] # [doc = " \"#;"] # [doc = " let mut bytes = Vec::new();"] # [doc = " let parser = Parser::new(markdown_str);"] # [doc = ""] # [doc = " html::write_html_io(Cursor::new(&mut bytes), parser);"] # [doc = ""] # [doc = " assert_eq!(&String::from_utf8_lossy(&bytes)[..], r#\"<h1>hello</h1>"] # [doc = " <ul>"] # [doc = " <li>alpha</li>"] # [doc = " <li>beta</li>"] # [doc = " </ul>"] # [doc = " \"#);"] # [doc = " ```"] # [cfg (feature = "std")] pub fn write_html_io < 'a , I , W > (writer : W , iter : I) -> std :: io :: Result < () > where I : Iterator < Item = Event < 'a > > , W : std :: io :: Write , { HtmlWriter :: new (iter , IoWriter (writer)) . run () }
};
}
