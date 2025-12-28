macro_rules! test_underscore {
    () => {
        # [test] fn test_underscore () { check (r#"
macro_rules! m { ($_:tt) => { ok!(); } }
m! { => }
"# , expect ! [[r#"
macro_rules! m { ($_:tt) => { ok!(); } }
ok!();
"#]] ,) ; }
    };
}

test_underscore!();