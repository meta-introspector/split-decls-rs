macro_rules! deps {
    () => {
        NodeIndex!();
        NodeIdentifiers!();
        IndexType!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        impl < Ix > Iterator for NodeIdentifiers < Ix > where Ix : IndexType , { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . r . next () . map (Ix :: new) } fn size_hint (& self) -> (usize , Option < usize >) { self . r . size_hint () } }
    };
}

impl_544!()