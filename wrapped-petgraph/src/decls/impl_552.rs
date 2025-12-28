macro_rules! deps {
    () => {
        NodeReferences!();
        IndexType!();
    };
}

macro_rules! impl_552 {
    () => {
        deps!();
        impl < N , Ix > DoubleEndedIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (i , weight) | (Ix :: new (i) , weight)) } }
    };
}

impl_552!()