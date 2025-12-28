macro_rules! mbe_are_not_attributes {
    () => {
        # [test] fn mbe_are_not_attributes () { check (r#"
macro_rules! error {
    () => {struct Bar}
}

#[error]
struct Foo;
"# , expect ! [[r##"
macro_rules! error {
    () => {struct Bar}
}

#[error]
struct Foo;
"##]] ,) }
    };
}

mbe_are_not_attributes!();