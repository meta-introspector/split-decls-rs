macro_rules! test_match_group_in_subtree {
    () => {
        # [test] fn test_match_group_in_subtree () { check (r#"
macro_rules! m {
    (fn $name:ident { $($i:ident)* } ) => ( fn $name() { $($i ();)* } );
}
m! { fn baz { a b } }
"# , expect ! [[r#"
macro_rules! m {
    (fn $name:ident { $($i:ident)* } ) => ( fn $name() { $($i ();)* } );
}
fn baz() {
    a();
    b();
}
"#]] ,) }
    };
}

test_match_group_in_subtree!()