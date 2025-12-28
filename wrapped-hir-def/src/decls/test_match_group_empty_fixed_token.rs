macro_rules! test_match_group_empty_fixed_token {
    () => {
        # [test] fn test_match_group_empty_fixed_token () { check (r#"
macro_rules! m {
    ($($i:ident)* #abc) => ( fn baz() { $($i ();)* } );
}
m!{#abc}
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident)* #abc) => ( fn baz() { $($i ();)* } );
}
fn baz() {}
"#]] ,) }
    };
}

test_match_group_empty_fixed_token!();