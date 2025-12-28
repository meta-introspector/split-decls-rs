macro_rules! test_type_path_is_transcribed_as_expr_path {
    () => {
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

test_type_path_is_transcribed_as_expr_path!()