// Generated macro for impl_141 (impl)
macro_rules! Depcrate_pathsimpl_141 {
() => {
// Module: crate::paths
// Provides: {"impl_141"}
// Dependencies: {}
impl Drop for TestIdGuard { fn drop (& mut self) { TEST_ID . with (| n | * n . borrow_mut () = None) ; } }
};
}
