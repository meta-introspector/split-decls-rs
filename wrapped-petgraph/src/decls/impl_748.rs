macro_rules! deps {
    () => {
        NodeReferences!();
        IndexType!();
        NodeIndex!();
    };
}

macro_rules! impl_748 {
    () => {
        deps!();
        impl < 'a , N , Ix > Iterator for NodeReferences < 'a , N , Ix > where Ix : IndexType , { type Item = (NodeIndex < Ix > , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (i , node) | (node_index (i) , & node . weight)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_748!()