// Generated macro for impl_435 (impl)
macro_rules! Depcrate_sparseimpl_435 {
() => {
// Module: crate::sparse
// Provides: {"impl_435"}
// Dependencies: {}
impl Deref for SparseSet { type Target = [usize] ; fn deref (& self) -> & Self :: Target { & self . dense [0 .. self . size] } }
};
}
