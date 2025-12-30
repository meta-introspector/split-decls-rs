// Generated macro for impl_302 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_unimapimpl_302 {
() => {
// Module: crate::borrow_tracker::tree_borrows::unimap
// Provides: {"impl_302"}
// Dependencies: {}
impl < 'a , V > UniEntry < 'a , V > { # [doc = " Insert in the map and get the value."] pub fn or_insert (& mut self , default : V) -> & mut V { if self . inner . is_none () { * self . inner = Some (default) ; } self . inner . as_mut () . unwrap () } pub fn get (& self) -> Option < & V > { self . inner . as_ref () } }
};
}
