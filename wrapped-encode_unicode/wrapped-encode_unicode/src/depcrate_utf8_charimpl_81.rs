// Generated macro for impl_81 (impl)
macro_rules! Depcrate_utf8_charimpl_81 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > Extend < & 'a Utf8Char > for String { fn extend < I : IntoIterator < Item = & 'a Utf8Char > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) } }
};
}
