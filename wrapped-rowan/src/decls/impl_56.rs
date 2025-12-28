macro_rules! deps {
    () => {
        NodeOrToken!();
        WalkEvent!();
        Language!();
        SyntaxNode!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < L : Language > fmt :: Debug for SyntaxNode < L > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { let mut level = 0 ; for event in self . preorder_with_tokens () { match event { WalkEvent :: Enter (element) => { for _ in 0 .. level { write ! (f , "  ") ? ; } match element { NodeOrToken :: Node (node) => writeln ! (f , "{:?}" , node) ? , NodeOrToken :: Token (token) => writeln ! (f , "{:?}" , token) ? , } level += 1 ; } WalkEvent :: Leave (_) => level -= 1 , } } assert_eq ! (level , 0) ; Ok (()) } else { write ! (f , "{:?}@{:?}" , self . kind () , self . text_range ()) } } }
    };
}

impl_56!();