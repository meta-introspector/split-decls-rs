macro_rules! deps {
    () => {
        NodeTrait!();
        GraphMap!();
        EdgeType!();
        Edges!();
    };
}

macro_rules! impl_897 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , S > Iterator for Edges < 'a , N , E , Ty , S > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , S : BuildHasher , { type Item = (N , N , & 'a E) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| b | { let a = self . from ; match self . edges . get (& GraphMap :: < N , E , Ty , S > :: edge_key (a , b)) { None => unreachable ! () , Some (edge) => (a , b , edge) , } }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_897!();