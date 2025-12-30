// Generated macro for debug_enabled (function)
macro_rules! Depcratedebug_enabled {
() => {
// Module: crate
// Provides: {"debug_enabled"}
// Dependencies: {}
fn debug_enabled () -> bool { static DEBUG_ENABLED : OnceLock < bool > = OnceLock :: new () ; * DEBUG_ENABLED . get_or_init (| | std :: env :: var_os ("CRITERION_DEBUG") . is_some ()) }
};
}
