// Generated macro for impl_169 (impl)
macro_rules! Depcrate_utf16_charimpl_169 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_169"}
// Dependencies: {}
# [cfg (feature = "std")] impl FromIterator < Utf16Char > for String { fn from_iter < I : IntoIterator < Item = Utf16Char > > (iter : I) -> Self { let mut s = String :: new () ; s . extend (iter) ; return s ; } }
};
}
