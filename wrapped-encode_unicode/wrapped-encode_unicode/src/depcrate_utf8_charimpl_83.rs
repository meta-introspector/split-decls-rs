// Generated macro for impl_83 (impl)
macro_rules! Depcrate_utf8_charimpl_83 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > FromIterator < & 'a Utf8Char > for String { fn from_iter < I : IntoIterator < Item = & 'a Utf8Char > > (iter : I) -> String { iter . into_iter () . cloned () . collect () } }
};
}
