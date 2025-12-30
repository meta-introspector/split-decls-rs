// Generated macro for string_continue (function)
macro_rules! Depcrate_cstr_testsstring_continue {
() => {
// Module: crate::cstr::tests
// Provides: {"string_continue"}
// Dependencies: {}
# [test] fn string_continue () { check ! (c"foo\
        bar" , true , None) ; check ! (c"foo\
bar" , true , None) ; check ! (c"foo\

        banana" , true , None) ; let lit = CStringLit :: parse ("c\"foo\\\n\t\n \n\tbar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , c"foobar") ; check ! (cr"foo\
        bar" , false , Some (0)) ; }
};
}
