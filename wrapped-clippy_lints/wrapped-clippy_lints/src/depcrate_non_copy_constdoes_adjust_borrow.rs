// Generated macro for does_adjust_borrow (function)
macro_rules! Depcrate_non_copy_constdoes_adjust_borrow {
() => {
// Module: crate::non_copy_const
// Provides: {"does_adjust_borrow"}
// Dependencies: {}
# [doc = " Checks if the adjustment causes a borrow of the original value. Returns"] # [doc = " `None` if the value is consumed instead of borrowed."] fn does_adjust_borrow (adjust : & Adjustment < '_ >) -> Option < BorrowCause > { match adjust . kind { Adjust :: Borrow (_) => Some (BorrowCause :: AutoBorrow) , Adjust :: Deref (Some (_)) => Some (BorrowCause :: AutoDeref) , _ => None , } }
};
}
