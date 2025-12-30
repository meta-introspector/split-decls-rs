// Generated macro for impl_77 (impl)
macro_rules! Depcrate_containersimpl_77 {
() => {
// Module: crate::containers
// Provides: {"impl_77"}
// Dependencies: {}
impl Drop for ContainerHandle { fn drop (& mut self) { if std :: env :: var_os ("CARGO_CONTAINER_TEST_KEEP") . is_some () { return ; } remove_if_exists (& self . name) ; } }
};
}
