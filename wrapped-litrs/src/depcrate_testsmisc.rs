// Generated macro for misc (function)
macro_rules! Depcrate_testsmisc {
() => {
// Module: crate::tests
// Provides: {"misc"}
// Dependencies: {}
# [test] fn misc () { assert_err_single ! (Literal :: parse ("0x44.5") , UnexpectedChar , 4 .. 6) ; assert_err_single ! (Literal :: parse ("a") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse (";") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("0;") , UnexpectedChar , 1) ; assert_err_single ! (Literal :: parse (" 0") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("0 ") , UnexpectedChar , 1) ; assert_err_single ! (Literal :: parse ("_") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("_3") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("a_123") , InvalidLiteral , None) ; assert_err_single ! (Literal :: parse ("B_123") , InvalidLiteral , None) ; }
};
}
