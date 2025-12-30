// Generated macro for disable_error_reporting (function)
macro_rules! Depcrate_runtestdisable_error_reporting {
() => {
// Module: crate::runtest
// Provides: {"disable_error_reporting"}
// Dependencies: {}
# [cfg (not (windows))] fn disable_error_reporting < F : FnOnce () -> R , R > (f : F) -> R { f () }
};
}
