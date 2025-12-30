// Generated macro for test_format_args_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_format_args_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_format_args_expand"}
// Dependencies: {}
# [test] fn test_format_args_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    format_args!("{} {:?}", arg1(a, b, c), arg2);
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    builtin #format_args ("{} {:?}", arg1(a, b, c), arg2);
}
"##]] ,) ; }
};
}
