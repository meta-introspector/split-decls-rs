// Generated macro for impl_82 (impl)
macro_rules! Depcrate_utf8_charimpl_82 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (feature = "std")] impl FromIterator < Utf8Char > for String { fn from_iter < I : IntoIterator < Item = Utf8Char > > (iter : I) -> String { let mut string = String :: new () ; string . extend (iter) ; return string ; } }
};
}
