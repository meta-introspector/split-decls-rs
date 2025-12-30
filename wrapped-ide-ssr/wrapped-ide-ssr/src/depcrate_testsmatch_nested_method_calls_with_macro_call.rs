// Generated macro for match_nested_method_calls_with_macro_call (function)
macro_rules! Depcrate_testsmatch_nested_method_calls_with_macro_call {
() => {
// Module: crate::tests
// Provides: {"match_nested_method_calls_with_macro_call"}
// Dependencies: {}
# [test] fn match_nested_method_calls_with_macro_call () { assert_matches ("$a.z().z().z()" , r#"
            macro_rules! m1 { ($a:expr) => {$a}; }
            fn f() {m1!(h().i().j().z().z().z().d().e())}"# , & ["h().i().j().z().z().z()"] ,) ; }
};
}
