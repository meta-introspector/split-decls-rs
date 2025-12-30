// Generated macro for BorrowCause (enum)
macro_rules! Depcrate_non_copy_constBorrowCause {
() => {
// Module: crate::non_copy_const
// Provides: {"BorrowCause"}
// Dependencies: {}
# [doc = " What operation caused a borrow to occur."] # [derive (Clone , Copy)] enum BorrowCause { Borrow , Deref , Index , AutoDeref , AutoBorrow , AutoDerefField , }
};
}
