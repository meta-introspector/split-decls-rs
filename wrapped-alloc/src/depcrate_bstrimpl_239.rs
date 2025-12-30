// Generated macro for impl_239 (impl)
macro_rules! Depcrate_bstrimpl_239 {
() => {
// Module: crate::bstr
// Provides: {"impl_239"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl From < Box < [u8] > > for Box < ByteStr > { # [inline] fn from (s : Box < [u8] >) -> Box < ByteStr > { unsafe { Box :: from_raw (Box :: into_raw (s) as _) } } }
};
}
