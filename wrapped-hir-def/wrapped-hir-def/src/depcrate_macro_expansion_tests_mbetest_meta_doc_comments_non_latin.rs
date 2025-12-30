// Generated macro for test_meta_doc_comments_non_latin (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_meta_doc_comments_non_latin {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_meta_doc_comments_non_latin"}
// Dependencies: {}
# [test] fn test_meta_doc_comments_non_latin () { check (r#"
macro_rules! m {
    ($(#[$ m:meta])+) => ( $(#[$m])+ fn bar() {} )
}
m! {
    /// 錦瑟無端五十弦，一弦一柱思華年。
    /**
        莊生曉夢迷蝴蝶，望帝春心託杜鵑。
    */
}
"# , expect ! [[r#"
macro_rules! m {
    ($(#[$ m:meta])+) => ( $(#[$m])+ fn bar() {} )
}
#[doc = r" 錦瑟無端五十弦，一弦一柱思華年。"]
#[doc = r"
        莊生曉夢迷蝴蝶，望帝春心託杜鵑。
    "] fn bar() {}
"#]] ,) ; }
};
}
