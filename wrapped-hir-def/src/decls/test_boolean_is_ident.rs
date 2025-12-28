macro_rules! test_boolean_is_ident {
    () => {
        # [test] fn test_boolean_is_ident () { check (r#"
macro_rules! m {
    ($lit0:literal, $lit1:literal) => { const VALUE: (bool, bool) = ($lit0, $lit1); };
}
m!(true, false);
"# , expect ! [[r#"
macro_rules! m {
    ($lit0:literal, $lit1:literal) => { const VALUE: (bool, bool) = ($lit0, $lit1); };
}
const VALUE: (bool, bool) = (true , false );
"#]] ,) ; }
    };
}

test_boolean_is_ident!();