// Generated macro for impl_1362 (impl)
macro_rules! Depcrate_sliceimpl_1362 {
() => {
// Module: crate::slice
// Provides: {"impl_1362"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Clone , A : Allocator > SpecCloneIntoVec < T , A > for [T] { default fn clone_into (& self , target : & mut Vec < T , A >) { target . truncate (self . len ()) ; let (init , tail) = self . split_at (target . len ()) ; target . clone_from_slice (init) ; target . extend_from_slice (tail) ; } }
};
}
