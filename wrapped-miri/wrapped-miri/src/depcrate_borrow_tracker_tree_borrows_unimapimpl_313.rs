// Generated macro for impl_313 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_unimapimpl_313 {
() => {
// Module: crate::borrow_tracker::tree_borrows::unimap
// Provides: {"impl_313"}
// Dependencies: {}
impl < V : PartialEq > UniValMap < V > { # [doc = " Exact equality of two maps."] # [doc = " Less accurate but faster than `equivalent`, mostly because"] # [doc = " of the fast path when the lengths are different."] pub fn identical (& self , other : & Self) -> bool { self . data == other . data } # [doc = " Equality up to trailing `None`s of two maps, i.e."] # [doc = " do they represent the same mapping ?"] pub fn equivalent (& self , other : & Self) -> bool { let min_len = self . data . len () . min (other . data . len ()) ; self . data [min_len ..] . iter () . all (Option :: is_none) && other . data [min_len ..] . iter () . all (Option :: is_none) && (self . data [.. min_len] == other . data [.. min_len]) } }
};
}
