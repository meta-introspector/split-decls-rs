// Generated macro for impl_68 (impl)
macro_rules! Depcrate_arcimpl_68 {
() => {
// Module: crate::arc
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl < T : Clone > From < & [T] > for Arc < [T] > { # [doc = " Allocates a reference-counted slice and fills it by cloning `v`'s items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " let original: &[i32] = &[1, 2, 3];"] # [doc = " let shared: Arc<[i32]> = Arc::from(original);"] # [doc = " assert_eq!(&[1, 2, 3], &shared[..]);"] # [doc = " ```"] # [inline] fn from (v : & [T]) -> Self { unsafe { Self :: from_iter_exact (v . iter () . cloned () , v . len ()) } } }
};
}
