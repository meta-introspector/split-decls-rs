// Generated macro for ToArcSlice (trait)
macro_rules! Depcrate_syncToArcSlice {
() => {
// Module: crate::sync
// Provides: {"ToArcSlice"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [doc = " Specialization trait used for collecting into `Arc<[T]>`."] trait ToArcSlice < T > : Iterator < Item = T > + Sized { fn to_arc_slice (self) -> Arc < [T] > ; }
};
}
