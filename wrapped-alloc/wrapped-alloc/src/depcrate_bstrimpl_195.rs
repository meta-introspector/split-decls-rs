// Generated macro for impl_195 (impl)
macro_rules! Depcrate_bstrimpl_195 {
() => {
// Module: crate::bstr
// Provides: {"impl_195"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl FromIterator < u8 > for ByteString { # [inline] fn from_iter < T : IntoIterator < Item = u8 > > (iter : T) -> Self { ByteString (iter . into_iter () . collect ()) } }
};
}
