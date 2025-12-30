// Generated macro for test_type_path_is_transcribed_as_expr_path (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_type_path_is_transcribed_as_expr_path {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_type_path_is_transcribed_as_expr_path"}
// Dependencies: {}
# [test] fn test_type_path_is_transcribed_as_expr_path () { check (r#"
macro_rules! m {
    ($p:path) => { let $p; }
}
fn test() {
    m!(S)
    m!(S<i32>)
    m!(S<S<i32>>)
    m!(S<{ module::CONST < 42 }>)
}
"# , expect ! [[r#"
macro_rules! m {
    ($p:path) => { let $p; }
}
fn test() {
    let S;
    let S:: <i32> ;
    let S:: <S:: <i32>> ;
    let S:: < {
        module::CONST<42
    }
    > ;
}
"#]] ,) ; }
};
}
