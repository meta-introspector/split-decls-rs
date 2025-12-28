macro_rules! test_file_expand {
    () => {
        # [test] fn test_file_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! file {() => {}}

fn main() { file!(); }
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! file {() => {}}

fn main() { "file"; }
"##]] ,) ; }
    };
}

test_file_expand!();