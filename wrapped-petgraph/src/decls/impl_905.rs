macro_rules! deps {
    () => {
        AllEdgesMut!();
        NodeTrait!();
        EdgeType!();
    };
}

macro_rules! impl_905 {
    () => {
        deps!();
        impl < 'a , N , E , Ty > DoubleEndedIterator for AllEdgesMut < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (& (n1 , n2) , weight) | (n1 , n2 , weight)) } }
    };
}

impl_905!()