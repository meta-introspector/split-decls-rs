macro_rules! deps {
    () => {
        Token!();
        FrontmatterAllowed!();
    };
}

macro_rules! characters {
    () => {
        deps!();
        # [test] fn characters () { check_lexing ("'a' ' ' '\\n'" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 4 }, len: 4 }
        "#]] ,) ; }
    };
}

characters!()