// Generated macro for test_format_args_expand_with_raw_strings (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_format_args_expand_with_raw_strings {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_format_args_expand_with_raw_strings"}
// Dependencies: {}
# [test] fn test_format_args_expand_with_raw_strings () { check (r##"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    format_args!(
        r#"{},mismatch,"{}","{}""#,
        location_csv_pat(db, &analysis, vfs, &sm, pat_id),
        mismatch.expected.display(db),
        mismatch.actual.display(db)
    );
}
"## , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    builtin #format_args (r#"{},mismatch,"{}","{}""#, location_csv_pat(db, &analysis, vfs, &sm, pat_id), mismatch.expected.display(db), mismatch.actual.display(db));
}
"##]] ,) ; }
};
}
