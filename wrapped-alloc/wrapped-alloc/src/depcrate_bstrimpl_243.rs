// Generated macro for impl_243 (impl)
macro_rules! Depcrate_bstrimpl_243 {
() => {
// Module: crate::bstr
// Provides: {"impl_243"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] # [cfg (all (not (no_rc) , not (no_sync) , target_has_atomic = "ptr"))] impl From < Arc < [u8] > > for Arc < ByteStr > { # [inline] fn from (s : Arc < [u8] >) -> Arc < ByteStr > { unsafe { Arc :: from_raw (Arc :: into_raw (s) as _) } } }
};
}
