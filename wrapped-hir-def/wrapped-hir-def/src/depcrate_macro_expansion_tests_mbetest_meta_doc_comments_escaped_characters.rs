// Generated macro for test_meta_doc_comments_escaped_characters (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_meta_doc_comments_escaped_characters {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_meta_doc_comments_escaped_characters"}
// Dependencies: {}
# [test] fn test_meta_doc_comments_escaped_characters () { check (r#"
macro_rules! m {
    ($(#[$m:meta])+) => ( $(#[$m])+ fn bar() {} )
}
m! {
    /// \ " '
}
"# , expect ! [[r##"
macro_rules! m {
    ($(#[$m:meta])+) => ( $(#[$m])+ fn bar() {} )
}
#[doc = r#" \ " '"#] fn bar() {}
"##]] ,) ; }
};
}
