// Generated macro for impl_238 (impl)
macro_rules! Depcrate_bstrimpl_238 {
() => {
// Module: crate::bstr
// Provides: {"impl_238"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < & 'a ByteStr > for Cow < 'a , ByteStr > { # [inline] fn from (s : & 'a ByteStr) -> Self { Cow :: Borrowed (s) } }
};
}
