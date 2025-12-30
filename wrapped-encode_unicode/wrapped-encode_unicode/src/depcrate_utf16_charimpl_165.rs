// Generated macro for impl_165 (impl)
macro_rules! Depcrate_utf16_charimpl_165 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_165"}
// Dependencies: {}
# [cfg (feature = "std")] impl FromIterator < Utf16Char > for Vec < u16 > { fn from_iter < I : IntoIterator < Item = Utf16Char > > (iter : I) -> Self { let mut vec = Vec :: new () ; vec . extend (iter) ; return vec ; } }
};
}
