macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
        NodeOrToken!();
        Children!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < L : Language > Serialize for Children < & '_ SyntaxNode < L > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_seq (None) ? ; self . 0 . children_with_tokens () . try_for_each (| element | match element { NodeOrToken :: Node (it) => state . serialize_element (& it) , NodeOrToken :: Token (it) => state . serialize_element (& it) , }) ? ; state . end () } }
    };
}

impl_169!();