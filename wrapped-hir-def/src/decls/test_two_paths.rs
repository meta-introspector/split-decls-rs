macro_rules! test_two_paths {
    () => {
        # [test] fn test_two_paths () { check (r#"
macro_rules! m {
    ($i:path, $j:path) => { fn foo() { let a = $ i; let b = $j; } }
}
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($i:path, $j:path) => { fn foo() { let a = $ i; let b = $j; } }
}
fn foo() {
    let a = foo;
    let b = bar;
}
"#]] ,) ; }
    };
}

test_two_paths!();