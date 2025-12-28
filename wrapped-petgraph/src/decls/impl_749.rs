macro_rules! deps {
    () => {
        IndexType!();
        NodeReferences!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        impl < N , Ix > DoubleEndedIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (i , node) | (node_index (i) , & node . weight)) } }
    };
}

impl_749!();