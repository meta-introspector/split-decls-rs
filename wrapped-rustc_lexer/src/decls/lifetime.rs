macro_rules! deps {
    () => {
        FrontmatterAllowed!();
        Token!();
    };
}

macro_rules! lifetime {
    () => {
        deps!();
        # [test] fn lifetime () { check_lexing ("'abc" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: Lifetime { starts_with_number: false }, len: 4 }
        "#]] ,) ; }
    };
}

lifetime!();