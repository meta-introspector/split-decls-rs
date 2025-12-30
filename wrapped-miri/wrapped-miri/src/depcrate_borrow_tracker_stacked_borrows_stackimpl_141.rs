// Generated macro for impl_141 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_stackimpl_141 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::stack
// Provides: {"impl_141"}
// Dependencies: {}
impl Stack { pub fn retain (& mut self , tags : & FxHashSet < BorTag >) { let mut first_removed = None ; let mut read_idx = 1 ; let mut write_idx = read_idx ; while read_idx < self . borrows . len () { let left = self . borrows [read_idx - 1] ; let this = self . borrows [read_idx] ; let should_keep = match this . perm () { Permission :: SharedReadWrite => tags . contains (& this . tag ()) , Permission :: Disabled => left . perm () == Permission :: SharedReadWrite , Permission :: Unique | Permission :: SharedReadOnly => left . perm () == Permission :: SharedReadWrite || tags . contains (& this . tag ()) , } ; if should_keep { if read_idx != write_idx { self . borrows [write_idx] = self . borrows [read_idx] ; } write_idx += 1 ; } else if first_removed . is_none () { first_removed = Some (read_idx) ; } read_idx += 1 ; } self . borrows . truncate (write_idx) ; # [cfg (not (feature = "stack-cache"))] let _unused = first_removed ; # [cfg (feature = "stack-cache")] if let Some (first_removed) = first_removed { if ! self . unique_range . is_empty () { self . unique_range = 0 .. self . len () ; } for i in 0 .. CACHE_LEN { if self . cache . idx [i] >= first_removed { self . cache . items [i] = self . borrows [0] ; self . cache . idx [i] = 0 ; } } } } }
};
}
