macro_rules! test_concat_expand {
    () => {
        # [test] fn test_concat_expand () { check (r##"
#[rustc_builtin_macro]
macro_rules! concat {}

fn main() { concat!("fo", "o", 0, r#""bar""#, "\n", false, '"', -4, - 4, '\0'); }
"## , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! concat {}

fn main() { "foo0\"bar\"\nfalse\"-4-4\u{0}"; }
"##]] ,) ; }
    };
}

test_concat_expand!();