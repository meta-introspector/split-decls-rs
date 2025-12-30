// Generated macro for impl_53 (impl)
macro_rules! Depcrate_testsimpl_53 {
() => {
// Module: crate::tests
// Provides: {"impl_53"}
// Dependencies: {}
impl Drop for SmallDroppable { fn drop (& mut self) { DROP_COUNTER . with (| c | c . set (c . get () + 1)) ; } }
};
}
