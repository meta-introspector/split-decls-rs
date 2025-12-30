// Generated macro for impl_197 (impl)
macro_rules! Depcrate_bstrimpl_197 {
() => {
// Module: crate::bstr
// Provides: {"impl_197"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > FromIterator < & 'a [u8] > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = & 'a [u8] > > (iter : T) -> Self { let mut buf = Vec :: new () ; for b in iter { buf . extend_from_slice (b) ; } ByteString (buf) } }
};
}
