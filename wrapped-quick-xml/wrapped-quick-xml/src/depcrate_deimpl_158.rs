// Generated macro for impl_158 (impl)
macro_rules! Depcrate_deimpl_158 {
() => {
// Module: crate::de
// Provides: {"impl_158"}
// Dependencies: {}
impl < R : BufRead > IoReader < R > { # [doc = " Returns the underlying XML reader."] # [doc = ""] # [doc = " ```"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " use serde::Deserialize;"] # [doc = " use std::io::Cursor;"] # [doc = " use quick_xml::de::Deserializer;"] # [doc = " use quick_xml::NsReader;"] # [doc = ""] # [doc = " #[derive(Deserialize)]"] # [doc = " struct SomeStruct {"] # [doc = "     field1: String,"] # [doc = "     field2: String,"] # [doc = " }"] # [doc = ""] # [doc = " // Try to deserialize from broken XML"] # [doc = " let mut de = Deserializer::from_reader(Cursor::new("] # [doc = "     \"<SomeStruct><field1><field2></SomeStruct>\""] # [doc = " //   0                           ^= 28        ^= 41"] # [doc = " ));"] # [doc = ""] # [doc = " let err = SomeStruct::deserialize(&mut de);"] # [doc = " assert!(err.is_err());"] # [doc = ""] # [doc = " let reader: &NsReader<Cursor<&str>> = de.get_ref().get_ref();"] # [doc = ""] # [doc = " assert_eq!(reader.error_position(), 28);"] # [doc = " assert_eq!(reader.buffer_position(), 41);"] # [doc = " ```"] pub const fn get_ref (& self) -> & NsReader < R > { & self . reader } }
};
}
