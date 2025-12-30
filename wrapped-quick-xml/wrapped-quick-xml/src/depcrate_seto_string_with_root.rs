// Generated macro for to_string_with_root (function)
macro_rules! Depcrate_seto_string_with_root {
() => {
// Module: crate::se
// Provides: {"to_string_with_root"}
// Dependencies: {}
# [doc = " Serialize struct into a `String` using specified root tag name."] # [doc = " `root_tag` should be valid [XML name], otherwise error is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::se::to_string_with_root;"] # [doc = " # use serde::Serialize;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " #[derive(Serialize)]"] # [doc = " struct Root<'a> {"] # [doc = "     #[serde(rename = \"@attribute\")]"] # [doc = "     attribute: &'a str,"] # [doc = "     element: &'a str,"] # [doc = "     #[serde(rename = \"$text\")]"] # [doc = "     text: &'a str,"] # [doc = " }"] # [doc = ""] # [doc = " let data = Root {"] # [doc = "     attribute: \"attribute content\","] # [doc = "     element: \"element content\","] # [doc = "     text: \"text content\","] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     to_string_with_root(\"top-level\", &data).unwrap(),"] # [doc = "     \"<top-level attribute=\\\"attribute content\\\">\\"] # [doc = "         <element>element content</element>\\"] # [doc = "         text content\\"] # [doc = "     </top-level>\""] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " [XML name]: https://www.w3.org/TR/xml11/#NT-Name"] pub fn to_string_with_root < T > (root_tag : & str , value : & T) -> Result < String , SeError > where T : ? Sized + Serialize , { let mut buffer = String :: new () ; to_writer_with_root (& mut buffer , root_tag , value) ? ; Ok (buffer) }
};
}
