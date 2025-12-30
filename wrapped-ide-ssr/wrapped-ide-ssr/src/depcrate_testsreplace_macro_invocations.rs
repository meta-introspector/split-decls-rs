// Generated macro for replace_macro_invocations (function)
macro_rules! Depcrate_testsreplace_macro_invocations {
() => {
// Module: crate::tests
// Provides: {"replace_macro_invocations"}
// Dependencies: {}
# [test] fn replace_macro_invocations () { assert_ssr_transform ("try_!($a) ==>> $a?" , "macro_rules! try_ {() => {}} fn f1() -> Result<(), E> {bar(try_!(foo()));}" , expect ! [["macro_rules! try_ {() => {}} fn f1() -> Result<(), E> {bar(foo()?);}"]] ,) ; }
};
}
