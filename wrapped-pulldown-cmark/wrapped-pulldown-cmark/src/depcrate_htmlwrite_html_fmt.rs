// Generated macro for write_html_fmt (function)
macro_rules! Depcrate_htmlwrite_html_fmt {
() => {
// Module: crate::html
// Provides: {"write_html_fmt"}
// Dependencies: {}
# [doc = " Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and"] # [doc = " write it into Unicode-accepting buffer or stream."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use pulldown_cmark::{html, Parser};"] # [doc = ""] # [doc = " let markdown_str = r#\""] # [doc = " hello"] # [doc = " ====="] # [doc = ""] # [doc = " * alpha"] # [doc = " * beta"] # [doc = " \"#;"] # [doc = " let mut buf = String::new();"] # [doc = " let parser = Parser::new(markdown_str);"] # [doc = ""] # [doc = " html::write_html_fmt(&mut buf, parser);"] # [doc = ""] # [doc = " assert_eq!(buf, r#\"<h1>hello</h1>"] # [doc = " <ul>"] # [doc = " <li>alpha</li>"] # [doc = " <li>beta</li>"] # [doc = " </ul>"] # [doc = " \"#);"] # [doc = " ```"] pub fn write_html_fmt < 'a , I , W > (writer : W , iter : I) -> core :: fmt :: Result where I : Iterator < Item = Event < 'a > > , W : core :: fmt :: Write , { HtmlWriter :: new (iter , FmtWriter (writer)) . run () }
};
}
