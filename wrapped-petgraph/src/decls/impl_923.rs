macro_rules! deps {
    () => {
        EdgeType!();
        NodeReferences!();
        NodeTrait!();
    };
}

macro_rules! impl_923 {
    () => {
        deps!();
        impl < 'a , N , E , Ty > Iterator for NodeReferences < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { type Item = (N , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (n , _) | (* n , n)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_923!()