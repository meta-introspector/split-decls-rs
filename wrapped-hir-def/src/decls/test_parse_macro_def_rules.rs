macro_rules! test_parse_macro_def_rules {
    () => {
        # [test] fn test_parse_macro_def_rules () { cov_mark :: check ! (parse_macro_def_rules) ; check (r#"
macro m {
    ($id:ident) => { fn $id() {} }
}
m!(bar);
"# , expect ! [[r#"
macro m {
    ($id:ident) => { fn $id() {} }
}
fn bar() {}
"#]] ,) ; }
    };
}

test_parse_macro_def_rules!();