macro_rules! test_expr {
    () => {
        # [test] fn test_expr () { check (r#"
macro_rules! m {
    ($e:expr) => { fn bar() { $e; } }
}

m! { 2 + 2 * baz(3).quux() }
"# , expect ! [[r#"
macro_rules! m {
    ($e:expr) => { fn bar() { $e; } }
}

fn bar() {
    (2+2*baz(3).quux());
}
"#]] ,) }
    };
}

test_expr!()