// Generated macro for overlapping_possible_matches (function)
macro_rules! Depcrate_testsoverlapping_possible_matches {
() => {
// Module: crate::tests
// Provides: {"overlapping_possible_matches"}
// Dependencies: {}
# [test] fn overlapping_possible_matches () { assert_matches ("foo(foo($a))" , "fn foo() {} fn main() {foo(foo(foo(foo(42))))}" , & ["foo(foo(42))" , "foo(foo(foo(foo(42))))"] ,) ; }
};
}
