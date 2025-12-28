macro_rules! test_meta_extended_key_value_attributes {
    () => {
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

test_meta_extended_key_value_attributes!();