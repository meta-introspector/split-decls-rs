// Generated macro for string_continue (function)
macro_rules! Depcrate_bytestr_testsstring_continue {
() => {
// Module: crate::bytestr::tests
// Provides: {"string_continue"}
// Dependencies: {}
# [test] fn string_continue () { check ! (b"foo\
        bar" , true , None) ; check ! (b"foo\
bar" , true , None) ; check ! (b"foo\

        banana" , true , None) ; let lit = ByteStringLit :: parse ("b\"foo\\\n\t\n \n\tbar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , b"foobar") ; check ! (br"foo\
        bar" , false , Some (0)) ; }
};
}
