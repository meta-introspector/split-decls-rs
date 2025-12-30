// Generated macro for miri_for_host (function)
macro_rules! Depcrate_utilmiri_for_host {
() => {
// Module: crate::util
// Provides: {"miri_for_host"}
// Dependencies: {}
pub fn miri_for_host () -> Command { let mut cmd = miri () ; cmd . env ("MIRI_BE_RUSTC" , "host") ; cmd }
};
}
