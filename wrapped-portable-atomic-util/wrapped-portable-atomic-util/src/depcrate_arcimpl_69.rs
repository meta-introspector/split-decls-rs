// Generated macro for impl_69 (impl)
macro_rules! Depcrate_arcimpl_69 {
() => {
// Module: crate::arc
// Provides: {"impl_69"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl From < & str > for Arc < str > { # [doc = " Allocates a reference-counted `str` and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let shared: Arc<str> = Arc::from(\"eggplant\");"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & str) -> Self { let arc = Arc :: < [u8] > :: from (v . as_bytes ()) ; unsafe { Self :: from_raw (Arc :: into_raw (arc) as * const str) } } }
};
}
