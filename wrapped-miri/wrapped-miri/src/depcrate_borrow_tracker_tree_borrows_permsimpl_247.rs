// Generated macro for impl_247 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_permsimpl_247 {
() => {
// Module: crate::borrow_tracker::tree_borrows::perms
// Provides: {"impl_247"}
// Dependencies: {}
impl PermTransition { # [doc = " All transitions created through normal means (using `perform_access`)"] # [doc = " should be possible, but the same is not guaranteed by construction of"] # [doc = " transitions inferred by diagnostics. This checks that a transition"] # [doc = " reconstructed by diagnostics is indeed one that could happen."] fn is_possible (self) -> bool { self . from <= self . to } pub fn is_noop (self) -> bool { self . from == self . to } # [doc = " Extract result of a transition (checks that the starting point matches)."] pub fn applied (self , starting_point : Permission) -> Option < Permission > { (starting_point . inner == self . from) . then_some (Permission { inner : self . to }) } # [doc = " Determines if this transition would disable the permission."] pub fn produces_disabled (self) -> bool { self . to == Disabled } }
};
}
