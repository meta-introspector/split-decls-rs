macro_rules! test_tt_group {
    () => {
        # [test] fn test_tt_group () { check (r#"
macro_rules! m { ($($tt:tt)*) => { $($tt)* } }
m! { fn foo() {} }"
"# , expect ! [[r#"
macro_rules! m { ($($tt:tt)*) => { $($tt)* } }
fn foo() {}"
"#]] ,) ; }
    };
}

test_tt_group!()