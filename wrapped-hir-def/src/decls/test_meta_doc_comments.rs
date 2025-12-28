macro_rules! test_meta_doc_comments {
    () => {
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

test_meta_doc_comments!();