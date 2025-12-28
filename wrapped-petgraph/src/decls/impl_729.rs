macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndices!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        impl < Ix : IndexType > DoubleEndedIterator for EdgeIndices < Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . r . next_back () . map (edge_index) } }
    };
}

impl_729!();