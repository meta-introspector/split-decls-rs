macro_rules! deps {
    () => {
        Token!();
        FrontmatterAllowed!();
    };
}

macro_rules! frontmatter_allowed {
    () => {
        deps!();
        # [test] fn frontmatter_allowed () { check_lexing (r#"
---cargo
[dependencies]
clap = "4"
---

fn main() {}
"# , FrontmatterAllowed :: Yes , expect ! [[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Frontmatter { has_invalid_preceding_whitespace: false, invalid_infostring: false }, len: 38 }
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

frontmatter_allowed!();