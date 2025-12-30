// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl FieldBytesEncoding < BignP256 > for U256 { fn decode_field_bytes (field_bytes : & FieldBytes) -> Self { U256 :: from_be_byte_array (* field_bytes) } fn encode_field_bytes (& self) -> FieldBytes { self . to_be_byte_array () } }
};
}
