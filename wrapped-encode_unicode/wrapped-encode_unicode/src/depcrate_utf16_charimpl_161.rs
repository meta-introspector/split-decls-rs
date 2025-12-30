// Generated macro for impl_161 (impl)
macro_rules! Depcrate_utf16_charimpl_161 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_161"}
// Dependencies: {}
impl From < Utf16Char > for char { fn from (uc : Utf16Char) -> char { char :: from_utf16_array_unchecked (uc . to_array ()) } }
};
}
