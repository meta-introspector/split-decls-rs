// Generated macro for test_meta_extended_key_value_attributes (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_meta_extended_key_value_attributes {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_meta_extended_key_value_attributes"}
// Dependencies: {}
# [test] fn test_meta_extended_key_value_attributes () { check (r#"
macro_rules! m {
    (#[$m:meta]) => ( #[$m] fn bar() {} )
}
m! { #[doc = concat!("The `", "bla", "` lang item.")] }
"# , expect ! [[r#"
macro_rules! m {
    (#[$m:meta]) => ( #[$m] fn bar() {} )
}
#[doc = concat!("The `", "bla", "` lang item.")] fn bar() {}
"#]] ,) ; }
};
}
