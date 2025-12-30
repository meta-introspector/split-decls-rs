// Generated macro for impl_196 (impl)
macro_rules! Depcrate_bstrimpl_196 {
() => {
// Module: crate::bstr
// Provides: {"impl_196"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > FromIterator < & 'a str > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = & 'a str > > (iter : T) -> Self { ByteString (iter . into_iter () . collect :: < String > () . into_bytes ()) } }
};
}
