// Generated macro for impl_192 (impl)
macro_rules! Depcrate_bstrimpl_192 {
() => {
// Module: crate::bstr
// Provides: {"impl_192"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < ByteString > for Cow < 'a , ByteStr > { # [inline] fn from (s : ByteString) -> Self { Cow :: Owned (s) } }
};
}
