macro_rules! deps {
    () => {
        SyntaxElement!();
        SyntaxKind!();
    };
}

macro_rules! SyntaxElementChildrenByKind {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct SyntaxElementChildrenByKind < F : Fn (SyntaxKind) -> bool > { next : Option < SyntaxElement > , matcher : F , }
    };
}

SyntaxElementChildrenByKind!()