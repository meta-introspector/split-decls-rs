macro_rules! test_stringify_expand {
    () => {
        # [test] fn test_stringify_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! stringify {() => {}}

fn main() {
    stringify!(
        a
        b
        c
    );
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! stringify {() => {}}

fn main() {
    "a b c";
}
"##]] ,) ; }
    };
}

test_stringify_expand!();