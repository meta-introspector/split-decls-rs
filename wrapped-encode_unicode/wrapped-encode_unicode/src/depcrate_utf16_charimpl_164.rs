// Generated macro for impl_164 (impl)
macro_rules! Depcrate_utf16_charimpl_164 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_164"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > Extend < & 'a Utf16Char > for Vec < u16 > { fn extend < I : IntoIterator < Item = & 'a Utf16Char > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) } }
};
}
