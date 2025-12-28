macro_rules! deps {
    () => {
        Token!();
        Phase!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Phase < '_ > { fn next_non_trivial (& mut self , code_it : & mut SyntaxElementChildren) -> Option < SyntaxElement > { loop { let c = code_it . next () ; if let Some (SyntaxElement :: Token (t)) = & c { self . record_ignored_comments (t) ; if t . kind () . is_trivia () { continue ; } } return c ; } } fn record_ignored_comments (& mut self , token : & SyntaxToken) { if token . kind () == SyntaxKind :: COMMENT && let Phase :: Second (match_out) = self && let Some (comment) = ast :: Comment :: cast (token . clone ()) { match_out . ignored_comments . push (comment) ; } } }
    };
}

impl_20!()