// Generated macro for string_continue (function)
macro_rules! Depcrate_string_testsstring_continue {
() => {
// Module: crate::string::tests
// Provides: {"string_continue"}
// Dependencies: {}
# [test] fn string_continue () { check ! ("నక్క\
        bar" , true , None) ; check ! ("foo\
🦊" , true , None) ; check ! ("foo\

        banana" , true , None) ; let lit = StringLit :: parse ("\"foo\\\n\t\n \n\tbar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , "foobar") ; let lit = StringLit :: parse ("\"foo\\\n\u{85}bar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , "foo\u{85}bar") ; let lit = StringLit :: parse ("\"foo\\\n\u{a0}bar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , "foo\u{a0}bar") ; check ! (r"foo\
        bar" , false , Some (0)) ; }
};
}
