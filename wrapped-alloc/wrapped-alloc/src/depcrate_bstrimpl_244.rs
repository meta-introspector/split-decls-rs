// Generated macro for impl_244 (impl)
macro_rules! Depcrate_bstrimpl_244 {
() => {
// Module: crate::bstr
// Provides: {"impl_244"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] # [cfg (all (not (no_rc) , not (no_sync) , target_has_atomic = "ptr"))] impl From < Arc < ByteStr > > for Arc < [u8] > { # [inline] fn from (s : Arc < ByteStr >) -> Arc < [u8] > { unsafe { Arc :: from_raw (Arc :: into_raw (s) as _) } } }
};
}
