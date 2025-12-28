macro_rules! test_meta {
    () => {
        # [test] fn test_meta () { check (r#"
macro_rules! m {
    ($m:meta) => ( #[$m] fn bar() {} )
}
m! { cfg(target_os = "windows") }
m! { hello::world }
"# , expect ! [[r#"
macro_rules! m {
    ($m:meta) => ( #[$m] fn bar() {} )
}
#[cfg(target_os = "windows")] fn bar() {}
#[hello::world] fn bar() {}
"#]] ,) ; }
    };
}

test_meta!();