// Generated macro for impl_173 (impl)
macro_rules! Depcrate_clone_checkoutimpl_173 {
() => {
// Module: crate::clone::checkout
// Provides: {"impl_173"}
// Dependencies: {}
# [doc = " Consumption"] impl PrepareCheckout { # [doc = " Persist the contained repository as is even if an error may have occurred when checking out the main working tree."] pub fn persist (mut self) -> Repository { self . repo . take () . expect ("present and consumed once") } }
};
}
