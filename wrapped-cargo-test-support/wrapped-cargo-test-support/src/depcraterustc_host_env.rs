// Generated macro for rustc_host_env (function)
macro_rules! Depcraterustc_host_env {
() => {
// Module: crate
// Provides: {"rustc_host_env"}
// Dependencies: {}
# [doc = " The host triple suitable for use in a cargo environment variable (uppercased)."] pub fn rustc_host_env () -> String { rustc_host () . to_uppercase () . replace ('-' , "_") }
};
}
