// Generated macro for replace_function_call (function)
macro_rules! Depcrate_testsreplace_function_call {
() => {
// Module: crate::tests
// Provides: {"replace_function_call"}
// Dependencies: {}
# [test] fn replace_function_call () { assert_ssr_transform ("foo() ==>> bar()" , "fn foo() {$0$0} fn bar() {} fn f1() {foo(); foo();}" , expect ! [["fn foo() {} fn bar() {} fn f1() {bar(); bar();}"]] ,) ; }
};
}
