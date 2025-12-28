macro_rules! test_macro_2_0_panic_2015 {
    () => {
        # [test] fn test_macro_2_0_panic_2015 () { check (r#"
macro panic_2015 {
    () => (),
    (bar) => (),
}
panic_2015!(bar);
"# , expect ! [[r#"
macro panic_2015 {
    () => (),
    (bar) => (),
}

"#]] ,) ; }
    };
}

test_macro_2_0_panic_2015!();