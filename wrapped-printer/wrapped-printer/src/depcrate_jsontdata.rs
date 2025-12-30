// Generated macro for Data (enum)
macro_rules! Depcrate_jsontData {
() => {
// Module: crate::jsont
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Data represents things that look like strings, but may actually not be"] # [doc = " valid UTF-8. To handle this, `Data` is serialized as an object with one"] # [doc = " of two keys: `text` (for valid UTF-8) or `bytes` (for invalid UTF-8)."] # [doc = ""] # [doc = " The happy path is valid UTF-8, which streams right through as-is, since"] # [doc = " it is natively supported by JSON. When invalid UTF-8 is found, then it is"] # [doc = " represented as arbitrary bytes and base64 encoded."] # [derive (Clone , Debug , Hash , PartialEq , Eq)] enum Data < 'a > { Text { text : Cow < 'a , str > } , Bytes { bytes : & 'a [u8] } , }
};
}
