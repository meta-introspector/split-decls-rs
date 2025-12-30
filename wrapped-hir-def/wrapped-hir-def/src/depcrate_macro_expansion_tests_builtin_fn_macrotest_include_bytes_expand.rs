// Generated macro for test_include_bytes_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_include_bytes_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_include_bytes_expand"}
// Dependencies: {}
# [test] fn test_include_bytes_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! include_bytes {
    ($file:expr) => {{ /* compiler built-in */ }};
    ($file:expr,) => {{ /* compiler built-in */ }};
}

fn main() { include_bytes("foo");include_bytes(r"foo"); }
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! include_bytes {
    ($file:expr) => {{ /* compiler built-in */ }};
    ($file:expr,) => {{ /* compiler built-in */ }};
}

fn main() { include_bytes("foo");include_bytes(r"foo"); }
"##]] ,) ; }
};
}
