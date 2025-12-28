macro_rules! deps {
    () => {
        IndexType!();
        NodeWeightsMut!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        impl < 'a , N , Ix > Iterator for NodeWeightsMut < 'a , N , Ix > where Ix : IndexType , { type Item = & 'a mut N ; fn next (& mut self) -> Option < & 'a mut N > { self . nodes . next () . map (| node | & mut node . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . nodes . size_hint () } }
    };
}

impl_707!();