macro_rules! deps {
    () => {
        EdgeIndices!();
        IndexType!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        impl < Ix : IndexType > DoubleEndedIterator for EdgeIndices < Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . r . next_back () . map (edge_index) } }
    };
}

impl_729!()