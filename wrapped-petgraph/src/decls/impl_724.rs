macro_rules! deps {
    () => {
        NodeIndex!();
        NodeIndices!();
        IndexType!();
    };
}

macro_rules! impl_724 {
    () => {
        deps!();
        impl < Ix : IndexType > Iterator for NodeIndices < Ix > { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . r . next () . map (node_index) } fn size_hint (& self) -> (usize , Option < usize >) { self . r . size_hint () } }
    };
}

impl_724!();