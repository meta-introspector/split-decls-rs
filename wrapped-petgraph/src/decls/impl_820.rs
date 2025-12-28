macro_rules! deps {
    () => {
        NodeReferences!();
        IndexType!();
    };
}

macro_rules! impl_820 {
    () => {
        deps!();
        impl < N , Ix > DoubleEndedIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . ex_rfind_map (| (i , node) | node . weight . as_ref () . map (move | w | (node_index (i) , w))) } }
    };
}

impl_820!()