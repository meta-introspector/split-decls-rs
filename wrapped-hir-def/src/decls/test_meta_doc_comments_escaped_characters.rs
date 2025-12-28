macro_rules! test_meta_doc_comments_escaped_characters {
    () => {
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

test_meta_doc_comments_escaped_characters!();