// Generated macro for impl_319 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_unimapimpl_319 {
() => {
// Module: crate::borrow_tracker::tree_borrows::unimap
// Provides: {"impl_319"}
// Dependencies: {}
impl < 'a , V > UniValMap < V > { # [doc = " Get a wrapper around a mutable access to the value corresponding to `idx`."] pub fn entry (& 'a mut self , idx : UniIndex) -> UniEntry < 'a , V > { self . extend_to_length (idx . idx . to_usize () + 1) ; UniEntry { inner : & mut self . data [idx . idx . to_usize ()] } } }
};
}
