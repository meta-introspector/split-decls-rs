macro_rules! deps {
    () => {
        FilterEdge!();
        EdgeFilteredNeighbors!();
        EdgeRef!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < G , F > Iterator for EdgeFilteredNeighbors < '_ , G , F > where F : FilterEdge < G :: EdgeRef > , G : IntoEdges , { type Item = G :: NodeId ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; (& mut self . iter) . filter_map (move | edge | { if f . include_edge (edge) { Some (edge . target ()) } else { None } }) . next () } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_150!();