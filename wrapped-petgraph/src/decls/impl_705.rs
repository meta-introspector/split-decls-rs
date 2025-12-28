macro_rules! deps {
    () => {
        IndexType!();
        NodeWeights!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl < 'a , N , Ix > Iterator for NodeWeights < 'a , N , Ix > where Ix : IndexType , { type Item = & 'a N ; fn next (& mut self) -> Option < & 'a N > { self . nodes . next () . map (| node | & node . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . nodes . size_hint () } }
    };
}

impl_705!()