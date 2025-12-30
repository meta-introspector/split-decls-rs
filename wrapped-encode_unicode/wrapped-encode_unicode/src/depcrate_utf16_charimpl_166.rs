// Generated macro for impl_166 (impl)
macro_rules! Depcrate_utf16_charimpl_166 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_166"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > FromIterator < & 'a Utf16Char > for Vec < u16 > { fn from_iter < I : IntoIterator < Item = & 'a Utf16Char > > (iter : I) -> Self { Self :: from_iter (iter . into_iter () . cloned ()) } }
};
}
