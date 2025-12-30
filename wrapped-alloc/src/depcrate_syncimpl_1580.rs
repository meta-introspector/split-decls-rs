// Generated macro for impl_1580 (impl)
macro_rules! Depcrate_syncimpl_1580 {
() => {
// Module: crate::sync
// Provides: {"impl_1580"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Clone > ArcFromSlice < T > for Arc < [T] > { # [inline] default fn from_slice (v : & [T]) -> Self { unsafe { Self :: from_iter_exact (v . iter () . cloned () , v . len ()) } } }
};
}
