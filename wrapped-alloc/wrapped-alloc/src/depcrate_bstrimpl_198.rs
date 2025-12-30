// Generated macro for impl_198 (impl)
macro_rules! Depcrate_bstrimpl_198 {
() => {
// Module: crate::bstr
// Provides: {"impl_198"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > FromIterator < & 'a ByteStr > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = & 'a ByteStr > > (iter : T) -> Self { let mut buf = Vec :: new () ; for b in iter { buf . extend_from_slice (& b . 0) ; } ByteString (buf) } }
};
}
