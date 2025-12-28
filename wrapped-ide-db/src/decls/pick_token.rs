macro_rules! pick_token {
    () => {
        pub fn pick_token < T : AstToken > (mut tokens : TokenAtOffset < SyntaxToken >) -> Option < T > { tokens . find_map (T :: cast) }
    };
}

pick_token!();