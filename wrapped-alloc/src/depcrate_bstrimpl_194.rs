// Generated macro for impl_194 (impl)
macro_rules! Depcrate_bstrimpl_194 {
() => {
// Module: crate::bstr
// Provides: {"impl_194"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl FromIterator < char > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = char > > (iter : T) -> Self { ByteString (iter . into_iter () . collect :: < String > () . into_bytes ()) } }
};
}
