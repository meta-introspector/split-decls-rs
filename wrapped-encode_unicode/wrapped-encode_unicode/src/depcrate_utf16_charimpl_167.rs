// Generated macro for impl_167 (impl)
macro_rules! Depcrate_utf16_charimpl_167 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_167"}
// Dependencies: {}
# [cfg (feature = "std")] impl Extend < Utf16Char > for String { fn extend < I : IntoIterator < Item = Utf16Char > > (& mut self , iter : I) { self . extend (iter . into_iter () . map (Utf8Char :: from)) ; } }
};
}
