// Generated macro for BorrowData (struct)
macro_rules! Depcrate_borrow_setBorrowData {
() => {
// Module: crate::borrow_set
// Provides: {"BorrowData"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct BorrowData < 'tcx > { # [doc = " Location where the borrow reservation starts."] # [doc = " In many cases, this will be equal to the activation location but not always."] pub (crate) reserve_location : Location , # [doc = " Location where the borrow is activated."] pub (crate) activation_location : TwoPhaseActivation , # [doc = " What kind of borrow this is"] pub (crate) kind : mir :: BorrowKind , # [doc = " The region for which this borrow is live"] pub (crate) region : RegionVid , # [doc = " Place from which we are borrowing"] pub (crate) borrowed_place : mir :: Place < 'tcx > , # [doc = " Place to which the borrow was stored"] pub (crate) assigned_place : mir :: Place < 'tcx > , }
};
}
