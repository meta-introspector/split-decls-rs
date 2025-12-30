// Generated macro for ssr_with_extra_space (function)
macro_rules! Depcrate_testsssr_with_extra_space {
() => {
// Module: crate::tests
// Provides: {"ssr_with_extra_space"}
// Dependencies: {}
# [test] fn ssr_with_extra_space () { assert_ssr_transform ("foo($x  ) +    bar() ==>> bar($x)" , "fn foo() {} fn bar() {} fn main() { foo(  5 )  +bar(   ) }" , expect ! [["fn foo() {} fn bar() {} fn main() { bar(5) }"]] ,) ; }
};
}
