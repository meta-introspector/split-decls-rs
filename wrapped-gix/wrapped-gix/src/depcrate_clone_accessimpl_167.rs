// Generated macro for impl_167 (impl)
macro_rules! Depcrate_clone_accessimpl_167 {
() => {
// Module: crate::clone::access
// Provides: {"impl_167"}
// Dependencies: {}
impl Drop for PrepareFetch { fn drop (& mut self) { if let Some (repo) = self . repo . take () { std :: fs :: remove_dir_all (repo . workdir () . unwrap_or_else (| | repo . path ())) . ok () ; } } }
};
}
