// Generated macro for test_option_env_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_option_env_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_option_env_expand"}
// Dependencies: {}
# [test] fn test_option_env_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! option_env {() => {}}

fn main() { option_env!("TEST_ENV_VAR"); }
"# , expect ! [[r#"
#[rustc_builtin_macro]
macro_rules! option_env {() => {}}

fn main() { $crate::option::Option::None:: < &str>; }
"#]] ,) ; }
};
}
