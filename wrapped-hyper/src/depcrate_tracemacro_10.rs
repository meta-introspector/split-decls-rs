// Generated macro for macro_10 (macro)
macro_rules! Depcrate_tracemacro_10 {
() => {
// Module: crate::trace
// Provides: {"macro_10"}
// Dependencies: {}
# [cfg (all (not (hyper_unstable_tracing) , feature = "tracing"))] compile_error ! ("\
    The `tracing` feature is unstable, and requires the \
    `RUSTFLAGS='--cfg hyper_unstable_tracing'` environment variable to be set.\
") ;
};
}
