macro_rules! deps {
    () => {
        SyntaxElement!();
        SyntaxKind!();
        SyntaxElementChildrenByKind!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < F : Fn (SyntaxKind) -> bool > Iterator for SyntaxElementChildrenByKind < F > { type Item = SyntaxElement ; fn next (& mut self) -> Option < SyntaxElement > { self . next . take () . inspect (| next | { self . next = next . next_sibling_or_token_by_kind (& self . matcher) ; }) } }
    };
}

impl_44!();