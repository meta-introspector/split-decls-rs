// Generated macro for assert_err (macro)
macro_rules! Depcrate_test_utilassert_err {
() => {
// Module: crate::test_util
// Provides: {"assert_err"}
// Dependencies: {}
macro_rules ! assert_err { ($ ty : ident , $ input : literal , $ kind : ident , $ ($ span : tt) +) => { assert_err_single ! ($ ty :: parse ($ input) , $ kind , $ ($ span) +) ; assert_err_single ! ($ crate :: Literal :: parse ($ input) , $ kind , $ ($ span) +) ; } ; }
};
}
