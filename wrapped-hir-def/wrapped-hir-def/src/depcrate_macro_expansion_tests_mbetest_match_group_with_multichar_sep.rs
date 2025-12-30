// Generated macro for test_match_group_with_multichar_sep (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_with_multichar_sep {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_with_multichar_sep"}
// Dependencies: {}
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
