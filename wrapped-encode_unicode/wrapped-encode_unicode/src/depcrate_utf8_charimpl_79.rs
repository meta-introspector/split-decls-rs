// Generated macro for impl_79 (impl)
macro_rules! Depcrate_utf8_charimpl_79 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > Extend < & 'a Utf8Char > for Vec < u8 > { fn extend < I : IntoIterator < Item = & 'a Utf8Char > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) } }
};
}
