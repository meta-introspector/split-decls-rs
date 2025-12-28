macro_rules! test_parse_macro_def_simple {
    () => {
        # [test] fn test_parse_macro_def_simple () { cov_mark :: check ! (parse_macro_def_simple) ; check (r#"
macro m($id:ident) { fn $id() {} }
m!(bar);
"# , expect ! [[r#"
macro m($id:ident) { fn $id() {} }
fn bar() {}
"#]] ,) ; }
    };
}

test_parse_macro_def_simple!()