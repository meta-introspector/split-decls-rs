macro_rules! test_compile_error_expand {
    () => {
        # [test] fn test_compile_error_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! compile_error {
    ($msg:expr) => ({ /* compiler built-in */ });
    ($msg:expr,) => ({ /* compiler built-in */ })
}

// This expands to nothing (since it's in item position), but emits an error.
compile_error!("error, with an escaped quote: \"");
compile_error!(r"this is a raw string");
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! compile_error {
    ($msg:expr) => ({ /* compiler built-in */ });
    ($msg:expr,) => ({ /* compiler built-in */ })
}

/* error: error, with an escaped quote: " */
/* error: this is a raw string */
"##]] ,) ; }
    };
}

test_compile_error_expand!();