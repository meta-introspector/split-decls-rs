macro_rules! test_env_expand {
    () => {
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

test_env_expand!()