macro_rules! test_match_group_zero_match {
    () => {
        # [test] fn test_match_group_zero_match () { check (r#"
macro_rules! m { ( $($i:ident)* ) => (); }
m!();
"# , expect ! [[r#"
macro_rules! m { ( $($i:ident)* ) => (); }

"#]] ,) ; }
    };
}

test_match_group_zero_match!()