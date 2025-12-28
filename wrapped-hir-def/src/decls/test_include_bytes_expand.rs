macro_rules! test_include_bytes_expand {
    () => {
        # [test] fn test_include_bytes_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! include_bytes {
    ($file:expr) => {{ /* compiler built-in */ }};
    ($file:expr,) => {{ /* compiler built-in */ }};
}

fn main() { include_bytes("foo");include_bytes(r"foo"); }
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! include_bytes {
    ($file:expr) => {{ /* compiler built-in */ }};
    ($file:expr,) => {{ /* compiler built-in */ }};
}

fn main() { include_bytes("foo");include_bytes(r"foo"); }
"##]] ,) ; }
    };
}

test_include_bytes_expand!()