// Generated macro for extensive_max_iterations (function)
macro_rules! Depcrate_run_cfgextensive_max_iterations {
() => {
// Module: crate::run_cfg
// Provides: {"extensive_max_iterations"}
// Dependencies: {}
# [doc = " Maximum number of iterations to run for a single routine."] # [doc = ""] # [doc = " The default value of one greater than `u32::MAX` allows testing single-argument `f32` routines"] # [doc = " and single- or double-argument `f16` routines exhaustively. `f64` and `f128` can't feasibly"] # [doc = " be tested exhaustively; however, [`EXTENSIVE_ITER_ENV`] can be set to run tests for multiple"] # [doc = " hours."] pub fn extensive_max_iterations () -> u64 { let default = 1 << 32 ; EXTENSIVE_ITER_OVERRIDE . unwrap_or (default) }
};
}
