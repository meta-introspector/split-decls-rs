// Generated macro for BorrowTrackerMethod (enum)
macro_rules! Depcrate_borrow_trackerBorrowTrackerMethod {
() => {
// Module: crate::borrow_tracker
// Provides: {"BorrowTrackerMethod"}
// Dependencies: {}
# [doc = " Which borrow tracking method to use"] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum BorrowTrackerMethod { # [doc = " Stacked Borrows, as implemented in borrow_tracker/stacked_borrows"] StackedBorrows , # [doc = " Tree borrows, as implemented in borrow_tracker/tree_borrows"] TreeBorrows (TreeBorrowsParams) , }
};
}
