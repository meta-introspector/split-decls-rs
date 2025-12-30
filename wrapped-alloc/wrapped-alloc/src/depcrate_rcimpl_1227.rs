// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_rcimpl_1227 {
() => {
// Module: crate::rc
// Provides: {"impl_1227"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Clone > RcFromSlice < T > for Rc < [T] > { # [inline] default fn from_slice (v : & [T]) -> Self { unsafe { Self :: from_iter_exact (v . iter () . cloned () , v . len ()) } } }
};
}
