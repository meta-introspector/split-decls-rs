macro_rules! deps {
    () => {
        NodeIndex!();
        NodeIdentifiers!();
        IndexType!();
    };
}

macro_rules! impl_993 {
    () => {
        deps!();
        impl < Ix : IndexType , S : BuildHasher > Iterator for NodeIdentifiers < '_ , Ix , S > { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (NodeIndex :: new) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_993!();