// Generated macro for impl_214 (impl)
macro_rules! Depcrate_vec_into_iterimpl_214 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_214"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Clone , A : Allocator + Clone > Clone for IntoIter < T , A > { fn clone (& self) -> Self { let mut vec = Vec :: < T , A > :: with_capacity_in (self . len () , (* self . alloc) . clone ()) ; vec . extend (self . as_slice () . iter () . cloned ()) ; vec . into_iter () } }
};
}
