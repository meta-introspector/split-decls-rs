// Generated macro for impl_84 (impl)
macro_rules! Depcrate_utf8_charimpl_84 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (feature = "std")] impl FromIterator < Utf8Char > for Vec < u8 > { fn from_iter < I : IntoIterator < Item = Utf8Char > > (iter : I) -> Self { iter . into_iter () . collect :: < String > () . into_bytes () } }
};
}
