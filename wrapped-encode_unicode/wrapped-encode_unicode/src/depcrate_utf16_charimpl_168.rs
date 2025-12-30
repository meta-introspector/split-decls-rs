// Generated macro for impl_168 (impl)
macro_rules! Depcrate_utf16_charimpl_168 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_168"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > Extend < & 'a Utf16Char > for String { fn extend < I : IntoIterator < Item = & 'a Utf16Char > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } }
};
}
