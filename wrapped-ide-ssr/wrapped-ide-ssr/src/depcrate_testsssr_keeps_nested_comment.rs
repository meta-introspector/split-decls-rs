// Generated macro for ssr_keeps_nested_comment (function)
macro_rules! Depcrate_testsssr_keeps_nested_comment {
() => {
// Module: crate::tests
// Provides: {"ssr_keeps_nested_comment"}
// Dependencies: {}
# [test] fn ssr_keeps_nested_comment () { assert_ssr_transform ("foo($x) ==>> bar($x)" , "fn foo() {} fn bar() {} fn main() { foo(other(5 /* using 5 */)) }" , expect ! [["fn foo() {} fn bar() {} fn main() { bar(other(5 /* using 5 */)) }"]] ,) }
};
}
