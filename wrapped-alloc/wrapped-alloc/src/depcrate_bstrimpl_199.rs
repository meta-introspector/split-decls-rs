// Generated macro for impl_199 (impl)
macro_rules! Depcrate_bstrimpl_199 {
() => {
// Module: crate::bstr
// Provides: {"impl_199"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl FromIterator < ByteString > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = ByteString > > (iter : T) -> Self { let mut buf = Vec :: new () ; for mut b in iter { buf . append (& mut b . 0) ; } ByteString (buf) } }
};
}
