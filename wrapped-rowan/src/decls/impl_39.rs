macro_rules! deps {
    () => {
        SyntaxNodeChildrenByKind!();
        SyntaxNode!();
        SyntaxKind!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < F : Fn (SyntaxKind) -> bool > Iterator for SyntaxNodeChildrenByKind < F > { type Item = SyntaxNode ; fn next (& mut self) -> Option < SyntaxNode > { self . next . take () . inspect (| next | { self . next = next . next_sibling_by_kind (& self . matcher) ; }) } }
    };
}

impl_39!()