macro_rules! test_stmt {
    () => {
        # [test] fn test_stmt () { check (r#"
macro_rules! m {
    ($s:stmt) => ( fn bar() { $s; } )
}
m! { 2 }
m! { let a = 0 }
"# , expect ! [[r#"
macro_rules! m {
    ($s:stmt) => ( fn bar() { $s; } )
}
fn bar() {
    2;
}
fn bar() {
    let a = 0;
}
"#]] ,) }
    };
}

test_stmt!()