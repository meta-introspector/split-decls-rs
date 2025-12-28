macro_rules! is_closing_token {
    () => {
        fn is_closing_token (kind : SyntaxKind) -> bool { kind == SyntaxKind :: R_PAREN || kind == SyntaxKind :: R_CURLY || kind == SyntaxKind :: R_BRACK }
    };
}

is_closing_token!();