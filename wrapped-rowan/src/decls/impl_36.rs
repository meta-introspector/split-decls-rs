macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxKind!();
        SyntaxNodeChildren!();
        SyntaxNodeChildrenByKind!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl SyntaxNodeChildren { fn new (parent : SyntaxNode) -> SyntaxNodeChildren { SyntaxNodeChildren { parent , next : None , next_initialized : false } } pub fn by_kind < F : Fn (SyntaxKind) -> bool > (self , matcher : F) -> SyntaxNodeChildrenByKind < F > { if ! self . next_initialized { SyntaxNodeChildrenByKind { next : self . parent . first_child_by_kind (& matcher) , matcher } } else { SyntaxNodeChildrenByKind { next : self . next . and_then (| node | { if matcher (node . kind ()) { Some (node) } else { node . next_sibling_by_kind (& matcher) } }) , matcher , } } } }
    };
}

impl_36!()