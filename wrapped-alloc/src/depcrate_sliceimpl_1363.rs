// Generated macro for impl_1363 (impl)
macro_rules! Depcrate_sliceimpl_1363 {
() => {
// Module: crate::slice
// Provides: {"impl_1363"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Copy , A : Allocator > SpecCloneIntoVec < T , A > for [T] { fn clone_into (& self , target : & mut Vec < T , A >) { target . clear () ; target . extend_from_slice (self) ; } }
};
}
