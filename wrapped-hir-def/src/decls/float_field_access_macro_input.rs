macro_rules! float_field_access_macro_input {
    () => {
        # [test] fn float_field_access_macro_input () { check (r#"
macro_rules! foo {
    ($expr:expr) => {
        fn foo() {
            $expr;
        }
    };
}
foo!(x .0.1);
foo!(x .2. 3);
foo!(x .4 .5);
"# , expect ! [[r#"
macro_rules! foo {
    ($expr:expr) => {
        fn foo() {
            $expr;
        }
    };
}
fn foo() {
    (x.0.1);
}
fn foo() {
    (x.2.3);
}
fn foo() {
    (x.4.5);
}
"#]] ,) ; }
    };
}

float_field_access_macro_input!()