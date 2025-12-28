macro_rules! deps {
    () => {
        NodeIdentifiers!();
        EdgeType!();
        NodeTrait!();
    };
}

macro_rules! impl_921 {
    () => {
        deps!();
        impl < 'a , N , E , Ty > Iterator for NodeIdentifiers < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { type Item = N ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (& n , _) | n) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_921!()