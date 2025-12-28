macro_rules! deps {
    () => {
        EdgeReferences!();
        EdgeReference!();
        IndexType!();
    };
}

macro_rules! impl_754 {
    () => {
        deps!();
        impl < 'a , E , Ix > Iterator for EdgeReferences < 'a , E , Ix > where Ix : IndexType , { type Item = EdgeReference < 'a , E , Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (i , edge) | EdgeReference { index : edge_index (i) , node : edge . node , weight : & edge . weight , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_754!();