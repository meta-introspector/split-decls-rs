// Generated macro for test_meta_doc_comments (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_meta_doc_comments {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_meta_doc_comments"}
// Dependencies: {}
# [test] fn test_meta_doc_comments () { check (r#"
macro_rules! m {
    ($(#[$m:meta])+) => ( $(#[$m])+ fn bar() {} )
}
m! {
    /// Single Line Doc 1
    /**
        MultiLines Doc
    */
}
"# , expect ! [[r#"
macro_rules! m {
    ($(#[$m:meta])+) => ( $(#[$m])+ fn bar() {} )
}
#[doc = r" Single Line Doc 1"]
#[doc = r"
        MultiLines Doc
    "] fn bar() {}
"#]] ,) ; }
};
}
