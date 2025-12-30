// Generated macro for AllocInit (enum)
macro_rules! Depcrate_raw_vecAllocInit {
() => {
// Module: crate::raw_vec
// Provides: {"AllocInit"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] enum AllocInit { # [doc = " The contents of the new memory are uninitialized."] Uninitialized , # [doc = " The new memory is guaranteed to be zeroed."] Zeroed , }
};
}
