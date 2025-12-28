macro_rules! test_match_group_pattern_with_multiple_statement_without_semi {
    () => {
        # [test] fn test_match_group_pattern_with_multiple_statement_without_semi () { check (r#"
macro_rules! m {
    ($($i:ident),*) => ( fn baz() { $($i() );* } );
}
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident),*) => ( fn baz() { $($i() );* } );
}
fn baz() {
    foo();
    bar()
}
"#]] ,) }
    };
}

test_match_group_pattern_with_multiple_statement_without_semi!()