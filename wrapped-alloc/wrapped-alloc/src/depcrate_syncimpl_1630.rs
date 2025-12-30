// Generated macro for impl_1630 (impl)
macro_rules! Depcrate_syncimpl_1630 {
() => {
// Module: crate::sync
// Provides: {"impl_1630"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "shared_from_slice" , since = "1.21.0")] impl < T : ? Sized , A : Allocator > From < Box < T , A > > for Arc < T , A > { # [doc = " Move a boxed object to a new, reference-counted allocation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::sync::Arc;"] # [doc = " let unique: Box<str> = Box::from(\"eggplant\");"] # [doc = " let shared: Arc<str> = Arc::from(unique);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : Box < T , A >) -> Arc < T , A > { Arc :: from_box_in (v) } }
};
}
