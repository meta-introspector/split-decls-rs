macro_rules! deps {
    () => {
        TreeMutator!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl TreeMutator { pub fn new (immutable : & SyntaxNode) -> TreeMutator { let immutable = immutable . ancestors () . last () . unwrap () ; let mutable_clone = immutable . clone_for_update () ; TreeMutator { immutable , mutable_clone } } pub fn make_mut < N : AstNode > (& self , node : & N) -> N { N :: cast (self . make_syntax_mut (node . syntax ())) . unwrap () } pub fn make_syntax_mut (& self , node : & SyntaxNode) -> SyntaxNode { let ptr = SyntaxNodePtr :: new (node) ; ptr . to_node (& self . mutable_clone) } }
    };
}

impl_190!()