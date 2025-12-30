// Generated macro for test_boolean_is_ident (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_boolean_is_ident {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_boolean_is_ident"}
// Dependencies: {}
# [test] fn test_boolean_is_ident () { check (r#"
macro_rules! m {
    ($lit0:literal, $lit1:literal) => { const VALUE: (bool, bool) = ($lit0, $lit1); };
}
m!(true, false);
"# , expect ! [[r#"
macro_rules! m {
    ($lit0:literal, $lit1:literal) => { const VALUE: (bool, bool) = ($lit0, $lit1); };
}
const VALUE: (bool, bool) = (true , false );
"#]] ,) ; }
};
}
