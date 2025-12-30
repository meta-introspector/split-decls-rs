// Generated macro for impl_25 (impl)
macro_rules! Depcrate_unindentimpl_25 {
() => {
// Module: crate::unindent
// Provides: {"impl_25"}
// Dependencies: {}
impl BytesExt for [u8] { fn lines (& self) -> Split < u8 , fn (& u8) -> bool > { fn is_newline (b : & u8) -> bool { * b == b'\n' } let bytestring = if self . starts_with (b"\r\n") { & self [1 ..] } else { self } ; bytestring . split (is_newline as fn (& u8) -> bool) } }
};
}
