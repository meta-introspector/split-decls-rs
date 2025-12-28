macro_rules! deps {
    () => {
        NodeFilteredEdgeReferences!();
        FilterNode!();
        EdgeRef!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < G , I , F > Iterator for NodeFilteredEdgeReferences < '_ , G , I , F > where F : FilterNode < G :: NodeId > , G : IntoEdgeReferences , I : Iterator < Item = G :: EdgeRef > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; self . iter . find (move | & edge | f . include_node (edge . source ()) && f . include_node (edge . target ())) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_130!()