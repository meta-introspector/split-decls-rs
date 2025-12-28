macro_rules! deps {
    () => {
        NodeIndex!();
        NodeReferences!();
        IndexType!();
    };
}

macro_rules! impl_996 {
    () => {
        deps!();
        impl < 'a , N : 'a , Ix : IndexType , S : BuildHasher > Iterator for NodeReferences < 'a , N , Ix , S > { type Item = (NodeIndex < Ix > , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| i | (NodeIndex :: new (i) , & self . nodes [i])) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_996!();