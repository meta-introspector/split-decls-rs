macro_rules! deps {
    () => {
        NodeKind!();
        MatchFailed!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl NodeKind { fn matches (& self , node : & SyntaxNode) -> Result < () , MatchFailed > { let ok = match self { Self :: Literal => { cov_mark :: hit ! (literal_constraint) ; ast :: Literal :: can_cast (node . kind ()) } } ; if ! ok { fail_match ! ("Code '{}' isn't of kind {:?}" , node . text () , self) ; } Ok (()) } }
    };
}

impl_26!()