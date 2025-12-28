macro_rules! test_option_env_expand {
    () => {
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

test_option_env_expand!();