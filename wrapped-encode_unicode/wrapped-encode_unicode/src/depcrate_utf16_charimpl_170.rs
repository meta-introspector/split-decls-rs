// Generated macro for impl_170 (impl)
macro_rules! Depcrate_utf16_charimpl_170 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_170"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > FromIterator < & 'a Utf16Char > for String { fn from_iter < I : IntoIterator < Item = & 'a Utf16Char > > (iter : I) -> Self { Self :: from_iter (iter . into_iter () . cloned ()) } }
};
}
