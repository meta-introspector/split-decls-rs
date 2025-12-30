// Generated macro for impl_71 (impl)
macro_rules! Depcrate_arcimpl_71 {
() => {
// Module: crate::arc
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : ? Sized > From < Box < T > > for Arc < T > { # [doc = " Move a boxed object to a new, reference-counted allocation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let unique: Box<str> = Box::from(\"eggplant\");"] # [doc = " let shared: Arc<str> = Arc::from(unique);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : Box < T >) -> Self { Self :: from_box (v) } }
};
}
