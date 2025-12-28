macro_rules! pick_best_token {
    () => {
        # [doc = " Picks the token with the highest rank returned by the passed in function."] pub fn pick_best_token (tokens : TokenAtOffset < SyntaxToken > , f : impl Fn (SyntaxKind) -> usize ,) -> Option < SyntaxToken > { tokens . max_by_key (move | t | f (t . kind ())) }
    };
}

pick_best_token!();