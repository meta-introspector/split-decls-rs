// Generated macro for impl_1629 (impl)
macro_rules! Depcrate_syncimpl_1629 {
() => {
// Module: crate::sync
// Provides: {"impl_1629"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl From < String > for Arc < str > { # [doc = " Allocates a reference-counted `str` and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let unique: String = \"eggplant\".to_owned();"] # [doc = " let shared: Arc<str> = Arc::from(unique);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : String) -> Arc < str > { Arc :: from (& v [..]) } }
};
}
