// Generated macro for replace_nested_function_calls (function)
macro_rules! Depcrate_testsreplace_nested_function_calls {
() => {
// Module: crate::tests
// Provides: {"replace_nested_function_calls"}
// Dependencies: {}
# [test] fn replace_nested_function_calls () { assert_ssr_transform ("foo($a) ==>> bar($a)" , "fn foo() {} fn bar() {} fn f1() {foo(foo(42))}" , expect ! [["fn foo() {} fn bar() {} fn f1() {bar(bar(42))}"]] ,) ; }
};
}
