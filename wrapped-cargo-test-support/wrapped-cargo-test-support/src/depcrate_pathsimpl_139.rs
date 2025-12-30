// Generated macro for impl_139 (impl)
macro_rules! Depcrate_pathsimpl_139 {
() => {
// Module: crate::paths
// Provides: {"impl_139"}
// Dependencies: {}
impl Drop for TestIdGuard { fn drop (& mut self) { TEST_ID . with (| n | * n . borrow_mut () = None) ; } }
};
}
