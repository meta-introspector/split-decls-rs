macro_rules! test_block {
    () => {
        # [test] fn test_block () { check (r#"
macro_rules! m { ($b:block) => { fn foo() $b } }
m! { { 1; } }
"# , expect ! [[r#"
macro_rules! m { ($b:block) => { fn foo() $b } }
fn foo() {
    1;
}
"#]] ,) ; }
    };
}

test_block!();