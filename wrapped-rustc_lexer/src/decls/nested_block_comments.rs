macro_rules! deps {
    () => {
        FrontmatterAllowed!();
        Token!();
    };
}

macro_rules! nested_block_comments {
    () => {
        deps!();
        # [test] fn nested_block_comments () { check_lexing ("/* /* */ */'a'" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 11 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
        "#]] ,) }
    };
}

nested_block_comments!();