// Generated macro for ArcFromSlice (trait)
macro_rules! Depcrate_syncArcFromSlice {
() => {
// Module: crate::sync
// Provides: {"ArcFromSlice"}
// Dependencies: {}
# [doc = " Specialization trait used for `From<&[T]>`."] # [cfg (not (no_global_oom_handling))] trait ArcFromSlice < T > { fn from_slice (slice : & [T]) -> Self ; }
};
}
