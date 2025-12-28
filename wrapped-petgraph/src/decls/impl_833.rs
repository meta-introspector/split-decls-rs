macro_rules! deps {
    () => {
        IndexType!();
        EdgeReference!();
        EdgeReferences!();
    };
}

macro_rules! impl_833 {
    () => {
        deps!();
        impl < E , Ix > DoubleEndedIterator for EdgeReferences < '_ , E , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , edge) | { edge . weight . as_ref () . map (move | weight | EdgeReference { index : edge_index (i) , node : edge . node , weight , }) }) } }
    };
}

impl_833!()