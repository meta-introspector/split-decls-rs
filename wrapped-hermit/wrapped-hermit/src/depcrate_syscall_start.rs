// Generated macro for _start (function)
macro_rules! Depcrate_syscall_start {
() => {
// Module: crate::syscall
// Provides: {"_start"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn _start (_argc : i32 , _argv : * const * const c_char) -> ! { extern "C" { fn runtime_entry (argc : i32 , argv : * const * const c_char , env : * const * const c_char) -> ! ; } let environ = core :: ptr :: null :: < * const c_char > () ; let argv = [c"dummy" . as_ptr ()] ; runtime_entry (1 , argv . as_ptr () , environ) }
};
}
