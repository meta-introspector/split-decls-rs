// Generated macro for impl_491 (impl)
macro_rules! Depcrate_sparseimpl_491 {
() => {
// Module: crate::sparse
// Provides: {"impl_491"}
// Dependencies: {}
impl Deref for SparseSet { type Target = [usize] ; fn deref (& self) -> & Self :: Target { & self . dense [0 .. self . size] } }
};
}
