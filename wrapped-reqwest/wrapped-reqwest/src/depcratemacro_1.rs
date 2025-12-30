// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (all (feature = "http3" , not (reqwest_unstable)))] compile_error ! ("\
    The `http3` feature is unstable, and requires the \
    `RUSTFLAGS='--cfg reqwest_unstable'` environment variable to be set.\
") ;
};
}
