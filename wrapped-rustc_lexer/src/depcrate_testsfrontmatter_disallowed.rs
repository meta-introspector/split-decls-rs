// Generated macro for frontmatter_disallowed (function)
macro_rules! Depcrate_testsfrontmatter_disallowed {
() => {
// Module: crate::tests
// Provides: {"frontmatter_disallowed"}
// Dependencies: {}
# [test] fn frontmatter_disallowed () { check_lexing (r#"
---cargo
[dependencies]
clap = "4"
---

fn main() {}
"# , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBracket, len: 1 }
            Token { kind: Ident, len: 12 }
            Token { kind: CloseBracket, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Whitespace, len: 2 }
            Token { kind: Ident, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
        "#]] ,) }
};
}
