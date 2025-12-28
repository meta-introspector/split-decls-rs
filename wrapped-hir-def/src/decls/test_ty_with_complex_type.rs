macro_rules! test_ty_with_complex_type {
    () => {
        # [test] fn test_ty_with_complex_type () { check (r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $ t {} )
}

m! { &'a Baz<u8> }

m! { extern "Rust" fn() -> Ret }
"# , expect ! [[r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $ t {} )
}

fn bar() -> &'a Baz<u8> {}

fn bar() -> extern "Rust" fn() -> Ret {}
"#]] ,) ; }
    };
}

test_ty_with_complex_type!()