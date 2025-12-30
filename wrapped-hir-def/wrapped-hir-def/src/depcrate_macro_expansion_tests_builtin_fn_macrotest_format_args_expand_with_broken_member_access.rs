// Generated macro for test_format_args_expand_with_broken_member_access (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_format_args_expand_with_broken_member_access {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_format_args_expand_with_broken_member_access"}
// Dependencies: {}
# [test] fn test_format_args_expand_with_broken_member_access () { check (r#"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    let _ =
        format_args!/*+errors*/("{} {:?}", a.);
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    let _ =
        /* parse error: expected field name or number */
builtin #format_args ("{} {:?}", a.);
}
"##]] ,) ; }
};
}
