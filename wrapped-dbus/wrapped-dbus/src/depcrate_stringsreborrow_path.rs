// Generated macro for reborrow_path (function)
macro_rules! Depcrate_stringsreborrow_path {
() => {
// Module: crate::strings
// Provides: {"reborrow_path"}
// Dependencies: {}
# [test] fn reborrow_path () { let p1 = Path :: from ("/valid") ; let p2 = p1 . clone () ; { let p2_borrow : & Path = & p2 ; let p3 = Path :: from (p2_borrow) ; assert_eq ! (p2 , p3) ; } assert_eq ! (p1 , p2) ; }
};
}
