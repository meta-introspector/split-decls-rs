// Generated macro for AllocState (enum)
macro_rules! Depcrate_borrow_trackerAllocState {
() => {
// Module: crate::borrow_tracker
// Provides: {"AllocState"}
// Dependencies: {}
# [doc = " Extra per-allocation data for borrow tracking"] # [derive (Debug , Clone)] pub enum AllocState { # [doc = " Data corresponding to Stacked Borrows"] StackedBorrows (Box < RefCell < stacked_borrows :: AllocState > >) , # [doc = " Data corresponding to Tree Borrows"] TreeBorrows (Box < RefCell < tree_borrows :: AllocState > >) , }
};
}
