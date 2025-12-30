// Generated macro for impl_83 (impl)
macro_rules! Depcrateimpl_83 {
() => {
// Module: crate
// Provides: {"impl_83"}
// Dependencies: {}
impl FieldBytesEncoding < Secp256k1 > for U256 { fn decode_field_bytes (field_bytes : & FieldBytes) -> Self { U256 :: from_be_byte_array (* field_bytes) } fn encode_field_bytes (& self) -> FieldBytes { self . to_be_byte_array () } }
};
}
