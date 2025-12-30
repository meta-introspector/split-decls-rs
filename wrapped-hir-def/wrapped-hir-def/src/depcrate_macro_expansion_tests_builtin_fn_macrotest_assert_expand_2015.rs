// Generated macro for test_assert_expand_2015 (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_assert_expand_2015 {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_assert_expand_2015"}
// Dependencies: {}
# [test] fn test_assert_expand_2015 () { check (r#"
//- minicore: assert
//- /main.rs edition:2015
fn main() {
    assert!(true, "{} {:?}", arg1(a, b, c), arg2);
}
"# , expect ! [[r#"
fn main() {
     {
        if !(true ) {
            $crate::panic::panic_2021!("{} {:?}", arg1(a, b, c), arg2);
        }
    };
}
"#]] ,) ; }
};
}
