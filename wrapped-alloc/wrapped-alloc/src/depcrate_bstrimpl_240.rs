// Generated macro for impl_240 (impl)
macro_rules! Depcrate_bstrimpl_240 {
() => {
// Module: crate::bstr
// Provides: {"impl_240"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl From < Box < ByteStr > > for Box < [u8] > { # [inline] fn from (s : Box < ByteStr >) -> Box < [u8] > { unsafe { Box :: from_raw (Box :: into_raw (s) as _) } } }
};
}
