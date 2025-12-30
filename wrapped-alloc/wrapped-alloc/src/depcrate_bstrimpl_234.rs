// Generated macro for impl_234 (impl)
macro_rules! Depcrate_bstrimpl_234 {
() => {
// Module: crate::bstr
// Provides: {"impl_234"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl ToOwned for ByteStr { type Owned = ByteString ; # [inline] fn to_owned (& self) -> ByteString { ByteString (self . 0 . to_vec ()) } }
};
}
