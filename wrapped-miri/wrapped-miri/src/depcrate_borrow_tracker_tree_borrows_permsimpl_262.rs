// Generated macro for impl_262 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_permsimpl_262 {
() => {
// Module: crate::borrow_tracker::tree_borrows::perms
// Provides: {"impl_262"}
// Dependencies: {}
# [cfg (test)] impl Permission { pub fn is_reserved_frz_with_conflicted (& self , expected_conflicted : bool) -> bool { match self . inner { ReservedFrz { conflicted } => conflicted == expected_conflicted , _ => false , } } }
};
}
