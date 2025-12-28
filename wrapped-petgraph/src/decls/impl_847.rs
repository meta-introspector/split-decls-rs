macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndices!();
    };
}

macro_rules! impl_847 {
    () => {
        deps!();
        impl < E , Ix : IndexType > DoubleEndedIterator for EdgeIndices < '_ , E , Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , node) | { if node . weight . is_some () { Some (edge_index (i)) } else { None } }) } }
    };
}

impl_847!()