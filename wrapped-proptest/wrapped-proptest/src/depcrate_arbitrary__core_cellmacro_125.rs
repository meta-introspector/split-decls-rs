// Generated macro for macro_125 (macro)
macro_rules! Depcrate_arbitrary__core_cellmacro_125 {
() => {
// Module: crate::arbitrary::_core::cell
// Provides: {"macro_125"}
// Dependencies: {}
lazy_just ! (BorrowError , || { # [cfg_attr (clippy , allow (let_and_return))] { let _rc = RefCell :: new (()) ; let _bm = _rc . borrow_mut () ; let _tb = _rc . try_borrow () ; let ret = _rc . try_borrow () . expect_err ("reborrowed RefCell") ; ret } }) ;
};
}
