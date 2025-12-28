macro_rules! deps {
    () => {
        NodeIndices!();
        IndexType!();
    };
}

macro_rules! impl_844 {
    () => {
        deps!();
        impl < N , Ix : IndexType > DoubleEndedIterator for NodeIndices < '_ , N , Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , node) | { if node . weight . is_some () { Some (node_index (i)) } else { None } }) } }
    };
}

impl_844!()