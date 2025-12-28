macro_rules! deps {
    () => {
        EdgeReference!();
        EdgeReferences!();
        IndexType!();
    };
}

macro_rules! impl_755 {
    () => {
        deps!();
        impl < E , Ix > DoubleEndedIterator for EdgeReferences < '_ , E , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (i , edge) | EdgeReference { index : edge_index (i) , node : edge . node , weight : & edge . weight , }) } }
    };
}

impl_755!()