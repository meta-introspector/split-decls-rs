// Generated macro for invalid_literals (function)
macro_rules! Depcrate_testsinvalid_literals {
() => {
// Module: crate::tests
// Provides: {"invalid_literals"}
// Dependencies: {}
# [test] fn invalid_literals () { assert_err_single ! (Literal :: parse (".") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("+") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("-") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("e") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("e8") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("f32") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("foo") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("inf") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("nan") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("NaN") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("NAN") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("_2.7") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse (".5") , InvalidLiteral , None) ; }
};
}
