macro_rules! test_tt_block {
    () => {
        # [test] fn test_tt_block () { check (r#"
macro_rules! m { ($tt:tt) => { fn foo() $tt } }
m! { { 1; } }
"# , expect ! [[r#"
macro_rules! m { ($tt:tt) => { fn foo() $tt } }
fn foo() {
    1;
}
"#]] ,) ; }
    };
}

test_tt_block!()