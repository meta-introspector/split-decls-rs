// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl FieldBytesEncoding < NistP256 > for U256 { fn decode_field_bytes (field_bytes : & FieldBytes) -> Self { U256 :: from_be_byte_array (* field_bytes) } fn encode_field_bytes (& self) -> FieldBytes { self . to_be_byte_array () } }
};
}
