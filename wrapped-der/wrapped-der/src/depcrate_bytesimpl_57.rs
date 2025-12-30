// Generated macro for impl_57 (impl)
macro_rules! Depcrate_bytesimpl_57 {
() => {
// Module: crate::bytes
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for & 'a BytesRef { type Error = Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : Header) -> Result < Self > { BytesRef :: new (reader . read_slice (header . length ()) ?) } }
};
}
