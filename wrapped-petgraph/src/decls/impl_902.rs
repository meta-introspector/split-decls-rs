macro_rules! deps {
    () => {
        NodeTrait!();
        EdgeType!();
        AllEdges!();
    };
}

macro_rules! impl_902 {
    () => {
        deps!();
        impl < 'a , N , E , Ty > DoubleEndedIterator for AllEdges < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (& (n1 , n2) , weight) | (n1 , n2 , weight)) } }
    };
}

impl_902!()