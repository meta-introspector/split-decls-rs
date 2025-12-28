macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxKind!();
    };
}

macro_rules! SyntaxNodeChildrenByKind {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct SyntaxNodeChildrenByKind < F : Fn (SyntaxKind) -> bool > { next : Option < SyntaxNode > , matcher : F , }
    };
}

SyntaxNodeChildrenByKind!();