// Generated macro for test_format_args_expand_with_comma_exprs (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_format_args_expand_with_comma_exprs {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_format_args_expand_with_comma_exprs"}
// Dependencies: {}
# [test] fn test_format_args_expand_with_comma_exprs () { check (r#"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    format_args!("{} {:?}", a::<A,B>(), b);
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    builtin #format_args ("{} {:?}", a::<A, B>(), b);
}
"##]] ,) ; }
};
}
