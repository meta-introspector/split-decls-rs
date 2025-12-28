macro_rules! deps {
    () => {
        EdgeIndex!();
        IndexType!();
        EdgeIndices!();
    };
}

macro_rules! impl_728 {
    () => {
        deps!();
        impl < Ix : IndexType > Iterator for EdgeIndices < Ix > { type Item = EdgeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . r . next () . map (edge_index) } fn size_hint (& self) -> (usize , Option < usize >) { self . r . size_hint () } }
    };
}

impl_728!()