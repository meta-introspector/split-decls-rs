// Generated macro for std_debug_assertions_enabled (function)
macro_rules! Depcrate_envstd_debug_assertions_enabled {
() => {
// Module: crate::env
// Provides: {"std_debug_assertions_enabled"}
// Dependencies: {}
# [doc = " Check if staged `std`-under-test was built with debug assertions."] # [track_caller] # [must_use] pub fn std_debug_assertions_enabled () -> bool { std :: env :: var_os ("__STD_DEBUG_ASSERTIONS_ENABLED") . is_some () }
};
}
