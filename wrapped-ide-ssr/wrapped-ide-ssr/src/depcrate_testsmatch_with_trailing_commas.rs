// Generated macro for match_with_trailing_commas (function)
macro_rules! Depcrate_testsmatch_with_trailing_commas {
() => {
// Module: crate::tests
// Provides: {"match_with_trailing_commas"}
// Dependencies: {}
# [test] fn match_with_trailing_commas () { assert_matches ("foo($a, $b)" , "fn foo() {} fn f() {foo(1, 2,);}" , & ["foo(1, 2,)"]) ; assert_matches ("Foo{$a, $b}" , "struct Foo {} fn f() {Foo{1, 2,};}" , & ["Foo{1, 2,}"]) ; assert_matches ("foo($a, $b,)" , "fn foo() {} fn f() {foo(1, 2);}" , & ["foo(1, 2)"]) ; assert_matches ("Foo{$a, $b,}" , "struct Foo {} fn f() {Foo{1, 2};}" , & ["Foo{1, 2}"]) ; }
};
}
