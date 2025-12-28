macro_rules! test_ty {
    () => {
        # [test] fn test_ty () { check (r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $t {} )
}
m! { Baz<u8> }
"# , expect ! [[r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $t {} )
}
fn bar() -> Baz<u8> {}
"#]] ,) }
    };
}

test_ty!();