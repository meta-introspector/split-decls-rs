macro_rules! expr_interpolation {
    () => {
        # [test] fn expr_interpolation () { check (r#"
macro_rules! m { ($expr:expr) => { map($expr) } }
fn f() {
    let _ = m!(x + foo);
}
"# , expect ! [[r#"
macro_rules! m { ($expr:expr) => { map($expr) } }
fn f() {
    let _ = map((x+foo));
}
"#]] ,) }
    };
}

expr_interpolation!();