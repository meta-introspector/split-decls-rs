// Generated macro for impl_1789 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1789 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1789"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "vec_into_iter_clone" , since = "1.8.0")] impl < T : Clone , A : Allocator + Clone > Clone for IntoIter < T , A > { fn clone (& self) -> Self { self . as_slice () . to_vec_in (self . alloc . deref () . clone ()) . into_iter () } }
};
}
