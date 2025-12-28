macro_rules! deps {
    () => {
        NodeIndices!();
        IndexType!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl < Ix : IndexType > DoubleEndedIterator for NodeIndices < Ix > { fn next_back (& mut self) -> Option < Self :: Item > { self . r . next_back () . map (node_index) } }
    };
}

impl_725!()