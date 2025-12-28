macro_rules! test_format_args_expand_with_comma_exprs {
    () => {
        # [test] fn test_format_args_expand_with_comma_exprs () { check (r#"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    format_args!("{} {:?}", a::<A,B>(), b);
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ })
}

fn main() {
    builtin #format_args ("{} {:?}", a::<A, B>(), b);
}
"##]] ,) ; }
    };
}

test_format_args_expand_with_comma_exprs!();