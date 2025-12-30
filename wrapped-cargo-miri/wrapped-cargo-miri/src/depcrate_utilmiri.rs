// Generated macro for miri (function)
macro_rules! Depcrate_utilmiri {
() => {
// Module: crate::util
// Provides: {"miri"}
// Dependencies: {}
pub fn miri () -> Command { let mut cmd = Command :: new (find_miri ()) ; cmd . env_remove ("MIRI_BE_RUSTC") ; cmd }
};
}
