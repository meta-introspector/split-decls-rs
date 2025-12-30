// Generated macro for mingw_unwinding (module)
macro_rules! Depcratemingw_unwinding {
() => {
// Module: crate
// Provides: {"mingw_unwinding"}
// Dependencies: {}
# [cfg (any (all (windows , target_env = "gnu") , target_os = "cygwin"))] mod mingw_unwinding { # [unsafe (no_mangle)] pub fn rust_eh_personality () { } # [unsafe (no_mangle)] pub fn rust_eh_unwind_resume () { } # [unsafe (no_mangle)] pub fn rust_eh_register_frames () { } # [unsafe (no_mangle)] pub fn rust_eh_unregister_frames () { } }
};
}
