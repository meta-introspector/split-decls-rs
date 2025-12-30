// Generated macro for test_env_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_env_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_env_expand"}
// Dependencies: {}
# [test] fn test_env_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! env {() => {}}

fn main() { env!("TEST_ENV_VAR"); }
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! env {() => {}}

fn main() { "UNRESOLVED_ENV_VAR"; }
"##]] ,) ; }
};
}
