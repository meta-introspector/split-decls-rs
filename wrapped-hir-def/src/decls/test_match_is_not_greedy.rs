macro_rules! test_match_is_not_greedy {
    () => {
        # [test] fn test_match_is_not_greedy () { check (r#"
macro_rules! foo {
    ($($i:ident $(,)*),*) => {};
}
foo!(a,b);
"# , expect ! [[r#"
macro_rules! foo {
    ($($i:ident $(,)*),*) => {};
}

"#]] ,) ; }
    };
}

test_match_is_not_greedy!();