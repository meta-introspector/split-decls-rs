// Generated macro for impl_112 (impl)
macro_rules! Depcrateimpl_112 {
() => {
// Module: crate
// Provides: {"impl_112"}
// Dependencies: {}
impl FieldBytesEncoding < Ed448 > for U448 { fn decode_field_bytes (field_bytes : & Ed448FieldBytes) -> Self { U448 :: from_le_slice (field_bytes) } fn encode_field_bytes (& self) -> Ed448FieldBytes { let mut data = Ed448FieldBytes :: default () ; data . copy_from_slice (& self . to_le_byte_array () [..]) ; data } }
};
}
