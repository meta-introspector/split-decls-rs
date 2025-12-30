// Generated macro for impl_1628 (impl)
macro_rules! Depcrate_syncimpl_1628 {
() => {
// Module: crate::sync
// Provides: {"impl_1628"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut str > for Arc < str > { # [doc = " Allocates a reference-counted `str` and copies `v` into it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let mut original = String::from(\"eggplant\");"] # [doc = " let original: &mut str = &mut original;"] # [doc = " let shared: Arc<str> = Arc::from(original);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & mut str) -> Arc < str > { Arc :: from (& * v) } }
};
}
