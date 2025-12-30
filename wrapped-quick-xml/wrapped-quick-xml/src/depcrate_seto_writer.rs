// Generated macro for to_writer (function)
macro_rules! Depcrate_seto_writer {
() => {
// Module: crate::se
// Provides: {"to_writer"}
// Dependencies: {}
# [doc = " Serialize struct into a `Write`r."] # [doc = ""] # [doc = " Returns the classification of the last written type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::se::to_writer;"] # [doc = " # use serde::Serialize;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " #[derive(Serialize)]"] # [doc = " struct Root<'a> {"] # [doc = "     #[serde(rename = \"@attribute\")]"] # [doc = "     attribute: &'a str,"] # [doc = "     element: &'a str,"] # [doc = "     #[serde(rename = \"$text\")]"] # [doc = "     text: &'a str,"] # [doc = " }"] # [doc = ""] # [doc = " let data = Root {"] # [doc = "     attribute: \"attribute content\","] # [doc = "     element: \"element content\","] # [doc = "     text: \"text content\","] # [doc = " };"] # [doc = ""] # [doc = " let mut buffer = String::new();"] # [doc = " to_writer(&mut buffer, &data).unwrap();"] # [doc = " assert_eq!("] # [doc = "     buffer,"] # [doc = "     // The root tag name is automatically deduced from the struct name"] # [doc = "     // This will not work for other types or struct with #[serde(flatten)] fields"] # [doc = "     \"<Root attribute=\\\"attribute content\\\">\\"] # [doc = "         <element>element content</element>\\"] # [doc = "         text content\\"] # [doc = "     </Root>\""] # [doc = " );"] # [doc = " ```"] pub fn to_writer < W , T > (mut writer : W , value : & T) -> Result < WriteResult , SeError > where W : Write , T : ? Sized + Serialize , { value . serialize (Serializer :: new (& mut writer)) }
};
}
