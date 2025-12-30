// Generated macro for runtime_entry (function)
macro_rules! Depcrateruntime_entry {
() => {
// Module: crate
// Provides: {"runtime_entry"}
// Dependencies: {}
# [cfg (test)] # [cfg (target_os = "none")] # [unsafe (no_mangle)] extern "C" fn runtime_entry (_argc : i32 , _argv : * const * const u8 , _env : * const * const u8) -> ! { println ! ("Executing hermit unittests. Any arguments are dropped") ; test_main () ; core_scheduler () . exit (0) }
};
}
