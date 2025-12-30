// Generated macro for assert_float_op_eq (macro)
macro_rules! Depcrate_tests_helperassert_float_op_eq {
() => {
// Module: crate::tests::helper
// Provides: {"assert_float_op_eq"}
// Dependencies: {}
# [cfg (feature = "float")] macro_rules ! assert_float_op_eq { ($ a : expr , $ b : expr $ (,) ?) => { { let a = $ a ; let b = $ b ; if a . is_nan () && b . is_nan () || a . is_infinite () && b . is_infinite () && a . is_sign_positive () == b . is_sign_positive () && a . is_sign_negative () == b . is_sign_negative () { } else { assert_eq ! (a , b) ; } } } ; }
};
}
