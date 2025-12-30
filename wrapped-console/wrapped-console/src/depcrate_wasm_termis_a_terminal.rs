// Generated macro for is_a_terminal (function)
macro_rules! Depcrate_wasm_termis_a_terminal {
() => {
// Module: crate::wasm_term
// Provides: {"is_a_terminal"}
// Dependencies: {}
# [inline] pub (crate) fn is_a_terminal (_out : & Term) -> bool { # [cfg (all (target_os = "wasi" , target_env = "p1"))] { use std :: os :: fd :: AsRawFd ; unsafe { libc :: isatty (_out . as_raw_fd ()) != 0 } } # [cfg (not (all (target_os = "wasi" , target_env = "p1")))] { false } }
};
}
