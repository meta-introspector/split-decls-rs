// Generated macro for rustc_debug_assertions_enabled (function)
macro_rules! Depcrate_envrustc_debug_assertions_enabled {
() => {
// Module: crate::env
// Provides: {"rustc_debug_assertions_enabled"}
// Dependencies: {}
# [doc = " Check if staged `rustc`-under-test was built with debug assertions."] # [track_caller] # [must_use] pub fn rustc_debug_assertions_enabled () -> bool { std :: env :: var_os ("__RUSTC_DEBUG_ASSERTIONS_ENABLED") . is_some () }
};
}
