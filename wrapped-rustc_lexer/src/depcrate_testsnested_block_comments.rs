// Generated macro for nested_block_comments (function)
macro_rules! Depcrate_testsnested_block_comments {
() => {
// Module: crate::tests
// Provides: {"nested_block_comments"}
// Dependencies: {}
# [test] fn nested_block_comments () { check_lexing ("/* /* */ */'a'" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 11 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
        "#]] ,) }
};
}
