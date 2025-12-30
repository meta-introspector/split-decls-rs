// Generated macro for impl_23 (impl)
macro_rules! Depcrate_displayimpl_23 {
() => {
// Module: crate::display
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a , 'b : 'a > super :: chunked_encoder :: Sink for FormatterSink < 'a , 'b > { type Error = fmt :: Error ; fn write_encoded_bytes (& mut self , encoded : & [u8]) -> Result < () , Self :: Error > { self . f . write_str (str :: from_utf8 (encoded) . expect ("base64 data was not utf8")) } }
};
}
