// Generated macro for impl_1366 (impl)
macro_rules! Depcrate_sliceimpl_1366 {
() => {
// Module: crate::slice
// Provides: {"impl_1366"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [unstable (issue = "none" , feature = "std_internals")] impl < T > sort :: stable :: BufGuard < T > for Vec < T > { fn with_capacity (capacity : usize) -> Self { Vec :: with_capacity (capacity) } fn as_uninit_slice_mut (& mut self) -> & mut [MaybeUninit < T >] { self . spare_capacity_mut () } }
};
}
