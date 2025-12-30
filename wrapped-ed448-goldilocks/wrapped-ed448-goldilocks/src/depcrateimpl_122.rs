// Generated macro for impl_122 (impl)
macro_rules! Depcrateimpl_122 {
() => {
// Module: crate
// Provides: {"impl_122"}
// Dependencies: {}
impl FieldBytesEncoding < Decaf448 > for U448 { fn decode_field_bytes (field_bytes : & Decaf448FieldBytes) -> Self { U448 :: from_le_slice (field_bytes) } fn encode_field_bytes (& self) -> Decaf448FieldBytes { let mut data = Decaf448FieldBytes :: default () ; data . copy_from_slice (& self . to_le_byte_array () [..]) ; data } }
};
}
