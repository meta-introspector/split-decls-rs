macro_rules! test_path_with_path {
    () => {
        # [test] fn test_path_with_path () { check (r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p::bar; } }
}
m! { foo }
"# , expect ! [[r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p::bar; } }
}
fn foo() {
    let a = foo::bar;
}
"#]] ,) ; }
    };
}

test_path_with_path!()