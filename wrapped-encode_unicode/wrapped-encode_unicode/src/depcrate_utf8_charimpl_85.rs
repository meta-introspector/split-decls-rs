// Generated macro for impl_85 (impl)
macro_rules! Depcrate_utf8_charimpl_85 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > FromIterator < & 'a Utf8Char > for Vec < u8 > { fn from_iter < I : IntoIterator < Item = & 'a Utf8Char > > (iter : I) -> Self { iter . into_iter () . cloned () . collect :: < String > () . into_bytes () } }
};
}
