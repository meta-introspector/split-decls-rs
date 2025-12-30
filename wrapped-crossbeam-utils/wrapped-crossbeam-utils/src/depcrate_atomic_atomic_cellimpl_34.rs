// Generated macro for impl_34 (impl)
macro_rules! Depcrate_atomic_atomic_cellimpl_34 {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"impl_34"}
// Dependencies: {}
impl < T > Drop for AtomicCell < T > { fn drop (& mut self) { if mem :: needs_drop :: < T > () { unsafe { self . as_ptr () . drop_in_place () ; } } } }
};
}
