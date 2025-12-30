// Generated macro for impl_191 (impl)
macro_rules! Depcrate_bstrimpl_191 {
() => {
// Module: crate::bstr
// Provides: {"impl_191"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < & 'a ByteStr > for ByteString { # [inline] fn from (s : & 'a ByteStr) -> Self { ByteString (s . 0 . to_vec ()) } }
};
}
