macro_rules! test_column_expand {
    () => {
        # [test] fn test_column_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! column {() => {}}

fn main() { column!(); }
"# , expect ! [[r#"
#[rustc_builtin_macro]
macro_rules! column {() => {}}

fn main() { 0u32; }
"#]] ,) ; }
    };
}

test_column_expand!();