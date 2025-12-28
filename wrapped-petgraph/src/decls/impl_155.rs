macro_rules! deps {
    () => {
        EdgeFilteredEdges!();
        FilterEdge!();
        EdgeRef!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < G , I , F > Iterator for EdgeFilteredEdges < '_ , G , I , F > where F : FilterEdge < G :: EdgeRef > , G : IntoEdgeReferences , I : Iterator < Item = G :: EdgeRef > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; self . iter . find (move | & edge | f . include_edge (edge)) } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_155!()