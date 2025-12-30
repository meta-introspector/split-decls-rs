// Generated macro for test_format_args_expand_eager (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_format_args_expand_eager {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_format_args_expand_eager"}
// Dependencies: {}
# [test] fn test_format_args_expand_eager () { check (r#"
#[rustc_builtin_macro]
macro_rules! concat {}

#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    format_args!(concat!("xxx{}y", "{:?}zzz"), 2, b);
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! concat {}

#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    builtin #format_args (concat!("xxx{}y", "{:?}zzz"), 2, b);
}
"##]] ,) ; }
};
}
