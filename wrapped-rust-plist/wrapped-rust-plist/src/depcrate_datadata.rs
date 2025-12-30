// Generated macro for Data (struct)
macro_rules! Depcrate_dataData {
() => {
// Module: crate::data
// Provides: {"Data"}
// Dependencies: {}
# [doc = " A byte buffer used for serialization to and from the plist data type."] # [doc = ""] # [doc = " You use it in types with derived `Serialize`/`Deserialize` traits."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate plist;"] # [doc = " #[macro_use]"] # [doc = " extern crate serde_derive;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " #[derive(Deserialize, Serialize)]"] # [doc = " struct Info {"] # [doc = "     blob: plist::Data,"] # [doc = " }"] # [doc = ""] # [doc = " let actual = Info { blob: plist::Data::new(vec![1, 2, 3, 4]) };"] # [doc = ""] # [doc = " let mut xml_byte_buffer: Vec<u8> = vec![];"] # [doc = " plist::to_writer_xml(&mut xml_byte_buffer, &actual)"] # [doc = "     .expect(\"serialize into xml\");"] # [doc = ""] # [doc = " let expected: Info = plist::from_reader_xml(xml_byte_buffer.as_slice())"] # [doc = "     .expect(\"deserialize from xml\");"] # [doc = ""] # [doc = " assert_eq!(actual.blob, expected.blob);"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq)] pub struct Data { inner : Vec < u8 > , }
};
}
