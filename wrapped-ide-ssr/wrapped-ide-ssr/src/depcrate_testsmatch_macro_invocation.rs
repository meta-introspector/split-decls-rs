// Generated macro for match_macro_invocation (function)
macro_rules! Depcrate_testsmatch_macro_invocation {
() => {
// Module: crate::tests
// Provides: {"match_macro_invocation"}
// Dependencies: {}
# [test] fn match_macro_invocation () { assert_matches ("foo!($a)" , "macro_rules! foo {() => {}} fn() {foo(foo!(foo()))}" , & ["foo!(foo())"] ,) ; assert_matches ("foo!(41, $a, 43)" , "macro_rules! foo {() => {}} fn() {foo!(41, 42, 43)}" , & ["foo!(41, 42, 43)"] ,) ; assert_no_match ("foo!(50, $a, 43)" , "macro_rules! foo {() => {}} fn() {foo!(41, 42, 43}") ; assert_no_match ("foo!(41, $a, 50)" , "macro_rules! foo {() => {}} fn() {foo!(41, 42, 43}") ; assert_matches ("foo!($a())" , "macro_rules! foo {() => {}} fn() {foo!(bar())}" , & ["foo!(bar())"] ,) ; }
};
}
