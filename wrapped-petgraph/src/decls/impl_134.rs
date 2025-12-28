macro_rules! deps {
    () => {
        EdgeRef!();
        FilterNode!();
        NodeFilteredEdges!();
        Direction!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < G , I , F > Iterator for NodeFilteredEdges < '_ , G , I , F > where F : FilterNode < G :: NodeId > , G : IntoEdges , I : Iterator < Item = G :: EdgeRef > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if ! self . include_source { None } else { let dir = self . dir ; let f = self . f ; self . iter . find (move | & edge | { f . include_node (match dir { Direction :: Outgoing => edge . target () , Direction :: Incoming => edge . source () , }) }) } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_134!();