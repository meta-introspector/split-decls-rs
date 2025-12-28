macro_rules! test_two_idents {
    () => {
        # [test] fn test_two_idents () { check (r#"
macro_rules! m {
    ($i:ident, $j:ident) => { fn foo() { let a = $i; let b = $j; } }
}
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($i:ident, $j:ident) => { fn foo() { let a = $i; let b = $j; } }
}
fn foo() {
    let a = foo;
    let b = bar;
}
"#]] ,) ; }
    };
}

test_two_idents!();