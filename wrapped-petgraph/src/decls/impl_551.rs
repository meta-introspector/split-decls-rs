macro_rules! deps {
    () => {
        NodeReferences!();
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_551 {
    () => {
        deps!();
        impl < 'a , N , Ix > Iterator for NodeReferences < 'a , N , Ix > where Ix : IndexType , { type Item = (NodeIndex < Ix > , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (i , weight) | (Ix :: new (i) , weight)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_551!();