macro_rules! test_concat_with_captured_expr {
    () => {
        # [test] fn test_concat_with_captured_expr () { check (r##"
#[rustc_builtin_macro]
macro_rules! concat {}

macro_rules! surprise {
    () => { "s" };
}

fn main() { concat!(surprise!()); }
"## , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! concat {}

macro_rules! surprise {
    () => { "s" };
}

fn main() { "s"; }
"##]] ,) ; }
    };
}

test_concat_with_captured_expr!()