// Generated macro for impl_174 (impl)
macro_rules! Depcrate_clone_checkoutimpl_174 {
() => {
// Module: crate::clone::checkout
// Provides: {"impl_174"}
// Dependencies: {}
impl Drop for PrepareCheckout { fn drop (& mut self) { if let Some (repo) = self . repo . take () { std :: fs :: remove_dir_all (repo . workdir () . unwrap_or_else (| | repo . path ())) . ok () ; } } }
};
}
