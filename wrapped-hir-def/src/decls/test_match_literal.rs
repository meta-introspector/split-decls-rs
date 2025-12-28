macro_rules! test_match_literal {
    () => {
        # [test] fn test_match_literal () { check (r#"
macro_rules! m {
    ('(') => { fn l_paren() {} }
}
m!['('];
"# , expect ! [[r#"
macro_rules! m {
    ('(') => { fn l_paren() {} }
}
fn l_paren() {}
"#]] ,) ; }
    };
}

test_match_literal!()