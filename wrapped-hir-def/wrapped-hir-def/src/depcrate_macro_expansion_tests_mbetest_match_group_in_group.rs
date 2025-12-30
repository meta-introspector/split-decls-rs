// Generated macro for test_match_group_in_group (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_in_group {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_in_group"}
// Dependencies: {}
# [test] fn test_match_group_in_group () { check (r#"
macro_rules! m {
    [ $( ( $($i:ident)* ) )* ] => [ ok![$( ( $($i)* ) )*]; ]
}
m! ( (a b) );
"# , expect ! [[r#"
macro_rules! m {
    [ $( ( $($i:ident)* ) )* ] => [ ok![$( ( $($i)* ) )*]; ]
}
ok![(a b)];
"#]] ,) }
};
}
