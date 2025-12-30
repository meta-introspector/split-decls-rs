// Generated macro for impl_70 (impl)
macro_rules! Depcrate_arcimpl_70 {
() => {
// Module: crate::arc
// Provides: {"impl_70"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl From < String > for Arc < str > { # [doc = " Allocates a reference-counted `str` and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let unique: String = \"eggplant\".to_owned();"] # [doc = " let shared: Arc<str> = Arc::from(unique);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : String) -> Self { Self :: from (& v [..]) } }
};
}
