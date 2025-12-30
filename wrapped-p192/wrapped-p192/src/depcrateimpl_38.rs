// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl FieldBytesEncoding < NistP192 > for U192 { fn decode_field_bytes (field_bytes : & FieldBytes) -> Self { U192 :: from_be_byte_array (* field_bytes) } fn encode_field_bytes (& self) -> FieldBytes { self . to_be_byte_array () } }
};
}
