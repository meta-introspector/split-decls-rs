// Generated macro for impl_77 (impl)
macro_rules! Depcrate_utf8_charimpl_77 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_77"}
// Dependencies: {}
impl IntoIterator for Utf8Char { type Item = u8 ; type IntoIter = Utf8Iterator ; # [doc = " Iterate over the byte values."] fn into_iter (self) -> Utf8Iterator { Utf8Iterator :: from (self) } }
};
}
