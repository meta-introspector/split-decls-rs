// Generated macro for handle_error (function)
macro_rules! Depcrate_raw_vechandle_error {
() => {
// Module: crate::raw_vec
// Provides: {"handle_error"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [cold] # [optimize (size)] # [track_caller] fn handle_error (e : TryReserveError) -> ! { match e . kind () { CapacityOverflow => capacity_overflow () , AllocError { layout , .. } => handle_alloc_error (layout) , } }
};
}
