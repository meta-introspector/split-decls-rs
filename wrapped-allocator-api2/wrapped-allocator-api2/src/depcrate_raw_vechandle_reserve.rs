// Generated macro for handle_reserve (function)
macro_rules! Depcrate_raw_vechandle_reserve {
() => {
// Module: crate::raw_vec
// Provides: {"handle_reserve"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [inline (always)] fn handle_reserve (result : Result < () , TryReserveError >) { match result . map_err (| e | e . kind ()) { Err (CapacityOverflow) => capacity_overflow () , Err (AllocError { layout , .. }) => handle_alloc_error (layout) , Ok (()) => { } } }
};
}
