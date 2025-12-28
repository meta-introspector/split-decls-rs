macro_rules! test_expr_with_attr {
    () => {
        # [test] fn test_expr_with_attr () { check (r#"
macro_rules! m { ($a:expr) => { ok!(); } }
m!(#[allow(a)]());
"# , expect ! [[r#"
macro_rules! m { ($a:expr) => { ok!(); } }
ok!();
"#]] ,) }
    };
}

test_expr_with_attr!()