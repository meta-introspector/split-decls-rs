macro_rules! test_quote_string {
    () => {
        # [test] fn test_quote_string () { check (r##"
#[rustc_builtin_macro]
macro_rules! stringify {}

fn main() { stringify!("hello"); }
"## , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! stringify {}

fn main() { "\"hello\""; }
"##]] ,) ; }
    };
}

test_quote_string!();