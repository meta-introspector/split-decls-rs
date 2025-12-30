// Generated macro for characters (function)
macro_rules! Depcrate_testscharacters {
() => {
// Module: crate::tests
// Provides: {"characters"}
// Dependencies: {}
# [test] fn characters () { check_lexing ("'a' ' ' '\\n'" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 4 }, len: 4 }
        "#]] ,) ; }
};
}
