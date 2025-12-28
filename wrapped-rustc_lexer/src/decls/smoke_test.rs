macro_rules! deps {
    () => {
        FrontmatterAllowed!();
        Token!();
    };
}

macro_rules! smoke_test {
    () => {
        deps!();
        # [test] fn smoke_test () { check_lexing ("/* my source file */ fn main() { println!(\"zebra\"); }\n" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: BlockComment { doc_style: None, terminated: true }, len: 20 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 7 }
            Token { kind: Bang, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 7 }, len: 7 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
        "#]] ,) }
    };
}

smoke_test!()