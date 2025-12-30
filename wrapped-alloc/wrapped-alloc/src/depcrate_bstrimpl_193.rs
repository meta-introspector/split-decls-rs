// Generated macro for impl_193 (impl)
macro_rules! Depcrate_bstrimpl_193 {
() => {
// Module: crate::bstr
// Provides: {"impl_193"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl < 'a > From < & 'a ByteString > for Cow < 'a , ByteStr > { # [inline] fn from (s : & 'a ByteString) -> Self { Cow :: Borrowed (s . as_bytestr ()) } }
};
}
