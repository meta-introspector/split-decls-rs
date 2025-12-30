// Generated macro for ssr_expected_spacing (function)
macro_rules! Depcrate_testsssr_expected_spacing {
() => {
// Module: crate::tests
// Provides: {"ssr_expected_spacing"}
// Dependencies: {}
# [test] fn ssr_expected_spacing () { assert_ssr_transform ("foo($x) + bar() ==>> bar($x)" , "fn foo() {} fn bar() {} fn main() { foo(5) + bar() }" , expect ! [["fn foo() {} fn bar() {} fn main() { bar(5) }"]] ,) ; }
};
}
