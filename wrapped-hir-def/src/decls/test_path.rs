macro_rules! test_path {
    () => {
        # [test] fn test_path () { check (r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p; } }
}

m! { foo }

m! { bar::<u8>::baz::<u8> }
"# , expect ! [[r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p; } }
}

fn foo() {
    let a = foo;
}

fn foo() {
    let a = bar::<u8>::baz::<u8> ;
}
"#]] ,) ; }
    };
}

test_path!();