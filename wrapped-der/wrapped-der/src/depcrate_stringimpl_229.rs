// Generated macro for impl_229 (impl)
macro_rules! Depcrate_stringimpl_229 {
() => {
// Module: crate::string
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for & 'a StringRef { type Error = Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : Header) -> Result < Self > { StringRef :: from_bytes (< & 'a BytesRef > :: decode_value (reader , header) ? . as_slice ()) } }
};
}
