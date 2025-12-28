macro_rules! test_match_group_with_multichar_sep {
    () => {
        # [test] fn test_match_group_with_multichar_sep () { check (r#"
macro_rules! m {
    (fn $name:ident { $($i:literal)* }) => ( fn $name() -> bool { $($i)&&* } );
}
m! (fn baz { true false } );
"# , expect ! [[r#"
macro_rules! m {
    (fn $name:ident { $($i:literal)* }) => ( fn $name() -> bool { $($i)&&* } );
}
fn baz() -> bool {
    true && false
}
"#]] ,) ; check (r#"
macro_rules! m {
    (fn $name:ident { $($i:literal)&&* }) => ( fn $name() -> bool { $($i)&&* } );
}
m! (fn baz { true && false } );
"# , expect ! [[r#"
macro_rules! m {
    (fn $name:ident { $($i:literal)&&* }) => ( fn $name() -> bool { $($i)&&* } );
}
fn baz() -> bool {
    true && false
}
"#]] ,) ; }
    };
}

test_match_group_with_multichar_sep!()