macro_rules! test_tt_with_composite_without_space {
    () => {
        # [test] fn test_tt_with_composite_without_space () { check (r#"
macro_rules! m { ($ op:tt, $j:path) => ( ok!(); ) }
m!(==,Foo::Bool)
"# , expect ! [[r#"
macro_rules! m { ($ op:tt, $j:path) => ( ok!(); ) }
ok!();
"#]] ,) ; }
    };
}

test_tt_with_composite_without_space!()