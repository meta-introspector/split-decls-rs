macro_rules! deps {
    () => {
        WalkEvent!();
        SyntaxNode!();
        NodeOrToken!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl fmt :: Display for SyntaxNode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . preorder_with_tokens () . filter_map (| event | match event { WalkEvent :: Enter (NodeOrToken :: Token (token)) => Some (token) , _ => None , }) . try_for_each (| it | fmt :: Display :: fmt (& it , f)) } }
    };
}

impl_28!();