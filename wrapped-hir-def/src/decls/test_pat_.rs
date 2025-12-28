macro_rules! test_pat_ {
    () => {
        # [test] fn test_pat_ () { check (r#"
macro_rules! m {
    ($p:pat) => { fn foo() { let $p; } }
}
m! { (a, b) }
"# , expect ! [[r#"
macro_rules! m {
    ($p:pat) => { fn foo() { let $p; } }
}
fn foo() {
    let (a, b);
}
"#]] ,) ; }
    };
}

test_pat_!();