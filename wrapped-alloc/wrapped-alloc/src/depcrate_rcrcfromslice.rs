// Generated macro for RcFromSlice (trait)
macro_rules! Depcrate_rcRcFromSlice {
() => {
// Module: crate::rc
// Provides: {"RcFromSlice"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [doc = " Specialization trait used for `From<&[T]>`."] trait RcFromSlice < T > { fn from_slice (slice : & [T]) -> Self ; }
};
}
