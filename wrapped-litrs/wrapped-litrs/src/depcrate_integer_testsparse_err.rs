// Generated macro for parse_err (function)
macro_rules! Depcrate_integer_testsparse_err {
() => {
// Module: crate::integer::tests
// Provides: {"parse_err"}
// Dependencies: {}
# [test] fn parse_err () { assert_err ! (IntegerLit , "" , Empty , None) ; assert_err_single ! (IntegerLit :: parse ("a") , DoesNotStartWithDigit , 0) ; assert_err_single ! (IntegerLit :: parse (";") , DoesNotStartWithDigit , 0) ; assert_err_single ! (IntegerLit :: parse ("0;") , UnexpectedChar , 1 .. 2) ; assert_err ! (IntegerLit , "0b" , NoDigits , 2 .. 2) ; assert_err_single ! (IntegerLit :: parse (" 0") , DoesNotStartWithDigit , 0) ; assert_err_single ! (IntegerLit :: parse ("0 ") , UnexpectedChar , 1) ; assert_err ! (IntegerLit , "0b3" , InvalidDigit , 2) ; assert_err_single ! (IntegerLit :: parse ("_") , DoesNotStartWithDigit , 0) ; assert_err_single ! (IntegerLit :: parse ("_3") , DoesNotStartWithDigit , 0) ; assert_err ! (IntegerLit , "0x44.5" , UnexpectedChar , 4 .. 6) ; assert_err_single ! (IntegerLit :: parse ("123em") , IntegerSuffixStartingWithE , 3) ; }
};
}
