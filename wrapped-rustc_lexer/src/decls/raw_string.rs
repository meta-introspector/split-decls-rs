macro_rules! deps {
    () => {
        Token!();
        FrontmatterAllowed!();
    };
}

macro_rules! raw_string {
    () => {
        deps!();
        # [test] fn raw_string () { check_lexing ("r###\"\"#a\\b\x00c\"\"###" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: Literal { kind: RawStr { n_hashes: Some(3) }, suffix_start: 17 }, len: 17 }
        "#]] ,) }
    };
}

raw_string!();