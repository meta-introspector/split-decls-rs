// Generated macro for cached (function)
macro_rules! Depcratecached {
() => {
// Module: crate
// Provides: {"cached"}
// Dependencies: {}
# [doc = " Returns true if the current environment is found to probably be a CI"] # [doc = " environment or service, and caches the result for future calls. If you"] # [doc = " expect the environment to change, use [uncached]."] pub fn cached () -> bool { INIT . call_once (| | IS_CI . store (uncached () , Ordering :: Relaxed)) ; IS_CI . load (Ordering :: Relaxed) }
};
}
