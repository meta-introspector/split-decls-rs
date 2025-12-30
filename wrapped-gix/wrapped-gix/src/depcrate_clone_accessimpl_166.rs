// Generated macro for impl_166 (impl)
macro_rules! Depcrate_clone_accessimpl_166 {
() => {
// Module: crate::clone::access
// Provides: {"impl_166"}
// Dependencies: {}
# [doc = " Consumption"] impl PrepareFetch { # [doc = " Persist the contained repository as is even if an error may have occurred when fetching from the remote."] pub fn persist (mut self) -> Repository { self . repo . take () . expect ("present and consumed once") } }
};
}
