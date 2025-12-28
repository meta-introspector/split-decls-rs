macro_rules! test_dollar_crate_lhs_is_not_meta {
    () => {
        # [test] fn test_dollar_crate_lhs_is_not_meta () { check (r#"
macro_rules! m {
    ($crate) => { err!(); };
    () => { ok!(); };
}
m!{}
"# , expect ! [[r#"
macro_rules! m {
    ($crate) => { err!(); };
    () => { ok!(); };
}
ok!();
"#]] ,) ; }
    };
}

test_dollar_crate_lhs_is_not_meta!();