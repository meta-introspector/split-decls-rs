// Generated macro for macro_126 (macro)
macro_rules! Depcrate_arbitrary__core_cellmacro_126 {
() => {
// Module: crate::arbitrary::_core::cell
// Provides: {"macro_126"}
// Dependencies: {}
lazy_just ! (BorrowMutError , || { # [cfg_attr (clippy , allow (let_and_return))] { let _rc = RefCell :: new (()) ; let _bm = _rc . borrow_mut () ; let _tb = _rc . try_borrow () ; let ret = _rc . try_borrow_mut () . expect_err ("reborrowed RefCell") ; ret } }) ;
};
}
