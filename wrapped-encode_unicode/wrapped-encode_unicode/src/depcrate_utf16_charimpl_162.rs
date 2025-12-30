// Generated macro for impl_162 (impl)
macro_rules! Depcrate_utf16_charimpl_162 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_162"}
// Dependencies: {}
impl IntoIterator for Utf16Char { type Item = u16 ; type IntoIter = Utf16Iterator ; # [doc = " Iterate over the units."] fn into_iter (self) -> Utf16Iterator { Utf16Iterator :: from (self) } }
};
}
