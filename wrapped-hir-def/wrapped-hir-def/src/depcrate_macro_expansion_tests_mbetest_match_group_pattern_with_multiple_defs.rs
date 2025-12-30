// Generated macro for test_match_group_pattern_with_multiple_defs (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_pattern_with_multiple_defs {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_pattern_with_multiple_defs"}
// Dependencies: {}
# [test] fn test_match_group_pattern_with_multiple_defs () { check (r#"
macro_rules! m {
    ($($i:ident),*) => ( impl Bar { $(fn $i() {})* } );
}
// +syntaxctxt
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident),*) => ( impl Bar { $(fn $i() {})* } );
}
impl#\14336# Bar#\14336# {#\14336#
    fn#\14336# foo#\ROOT2024#(#\14336#)#\14336# {#\14336#}#\14336#
    fn#\14336# bar#\ROOT2024#(#\14336#)#\14336# {#\14336#}#\14336#
}#\14336#
"#]] ,) ; }
};
}
