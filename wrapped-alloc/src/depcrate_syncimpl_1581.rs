// Generated macro for impl_1581 (impl)
macro_rules! Depcrate_syncimpl_1581 {
() => {
// Module: crate::sync
// Provides: {"impl_1581"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Copy > ArcFromSlice < T > for Arc < [T] > { # [inline] fn from_slice (v : & [T]) -> Self { unsafe { Arc :: copy_from_slice (v) } } }
};
}
