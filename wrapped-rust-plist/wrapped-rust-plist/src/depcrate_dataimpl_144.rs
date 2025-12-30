// Generated macro for impl_144 (impl)
macro_rules! Depcrate_dataimpl_144 {
() => {
// Module: crate::data
// Provides: {"impl_144"}
// Dependencies: {}
impl Data { # [doc = " Creates a new `Data` from vec of bytes."] pub fn new (bytes : Vec < u8 >) -> Self { Data { inner : bytes } } # [doc = " Create a `Data` object from an XML plist (Base-64) encoded string."] pub fn from_xml_format (b64_str : & str) -> Result < Self , InvalidXmlData > { BASE64_STANDARD . decode (b64_str) . map_err (InvalidXmlData) . map (Data :: new) } # [doc = " Converts the `Data` to an XML plist (Base-64) string."] pub fn to_xml_format (& self) -> String { xml_encode_data_base64 (& self . inner) } }
};
}
