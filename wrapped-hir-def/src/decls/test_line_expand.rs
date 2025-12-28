macro_rules! test_line_expand {
    () => {
        # [test] fn test_line_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! line {() => {}}

fn main() { line!() }
"# , expect ! [[r#"
#[rustc_builtin_macro]
macro_rules! line {() => {}}

fn main() { 0u32 }
"#]] ,) ; }
    };
}

test_line_expand!()