// Generated macro for impl_1627 (impl)
macro_rules! Depcrate_syncimpl_1627 {
() => {
// Module: crate::sync
// Provides: {"impl_1627"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl From < & str > for Arc < str > { # [doc = " Allocates a reference-counted `str` and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let shared: Arc<str> = Arc::from(\"eggplant\");"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & str) -> Arc < str > { let arc = Arc :: < [u8] > :: from (v . as_bytes ()) ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const str) } } }
};
}
