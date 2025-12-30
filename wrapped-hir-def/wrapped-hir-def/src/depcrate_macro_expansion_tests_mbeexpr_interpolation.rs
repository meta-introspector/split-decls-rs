// Generated macro for expr_interpolation (function)
macro_rules! Depcrate_macro_expansion_tests_mbeexpr_interpolation {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"expr_interpolation"}
// Dependencies: {}
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
