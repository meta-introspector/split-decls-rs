// Generated macro for impl_1364 (impl)
macro_rules! Depcrate_sliceimpl_1364 {
() => {
// Module: crate::slice
// Provides: {"impl_1364"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone > ToOwned for [T] { type Owned = Vec < T > ; fn to_owned (& self) -> Vec < T > { self . to_vec () } fn clone_into (& self , target : & mut Vec < T >) { SpecCloneIntoVec :: clone_into (self , target) ; } }
};
}
