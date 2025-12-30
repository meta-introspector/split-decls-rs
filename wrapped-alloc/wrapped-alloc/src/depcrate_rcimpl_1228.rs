// Generated macro for impl_1228 (impl)
macro_rules! Depcrate_rcimpl_1228 {
() => {
// Module: crate::rc
// Provides: {"impl_1228"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Copy > RcFromSlice < T > for Rc < [T] > { # [inline] fn from_slice (v : & [T]) -> Self { unsafe { Rc :: copy_from_slice (v) } } }
};
}
