// Generated macro for impl_348 (impl)
macro_rules! Depcrate_borrow_trackerimpl_348 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_348"}
// Dependencies: {}
impl machine :: AllocExtra < '_ > { # [track_caller] pub fn borrow_tracker_sb (& self) -> & RefCell < stacked_borrows :: AllocState > { match self . borrow_tracker { Some (AllocState :: StackedBorrows (ref sb)) => sb , _ => panic ! ("expected Stacked Borrows borrow tracking, got something else") , } } # [track_caller] pub fn borrow_tracker_sb_mut (& mut self) -> & mut RefCell < stacked_borrows :: AllocState > { match self . borrow_tracker { Some (AllocState :: StackedBorrows (ref mut sb)) => sb , _ => panic ! ("expected Stacked Borrows borrow tracking, got something else") , } } # [track_caller] pub fn borrow_tracker_tb (& self) -> & RefCell < tree_borrows :: AllocState > { match self . borrow_tracker { Some (AllocState :: TreeBorrows (ref tb)) => tb , _ => panic ! ("expected Tree Borrows borrow tracking, got something else") , } } }
};
}
