// Generated macro for to_string (function)
macro_rules! Depcrate_seto_string {
() => {
// Module: crate::se
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " Serialize struct into a `String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::se::to_string;"] # [doc = " # use serde::Serialize;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " #[derive(Serialize)]"] # [doc = " struct Root<'a> {"] # [doc = "     #[serde(rename = \"@attribute\")]"] # [doc = "     attribute: &'a str,"] # [doc = "     element: &'a str,"] # [doc = "     #[serde(rename = \"$text\")]"] # [doc = "     text: &'a str,"] # [doc = " }"] # [doc = ""] # [doc = " let data = Root {"] # [doc = "     attribute: \"attribute content\","] # [doc = "     element: \"element content\","] # [doc = "     text: \"text content\","] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     to_string(&data).unwrap(),"] # [doc = "     // The root tag name is automatically deduced from the struct name"] # [doc = "     // This will not work for other types or struct with #[serde(flatten)] fields"] # [doc = "     \"<Root attribute=\\\"attribute content\\\">\\"] # [doc = "         <element>element content</element>\\"] # [doc = "         text content\\"] # [doc = "     </Root>\""] # [doc = " );"] # [doc = " ```"] pub fn to_string < T > (value : & T) -> Result < String , SeError > where T : ? Sized + Serialize , { let mut buffer = String :: new () ; to_writer (& mut buffer , value) ? ; Ok (buffer) }
};
}
