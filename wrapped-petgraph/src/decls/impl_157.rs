macro_rules! deps {
    () => {
        EdgeRef!();
        EdgeFilteredNeighborsDirected!();
        FilterEdge!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < G , F > Iterator for EdgeFilteredNeighborsDirected < '_ , G , F > where F : FilterEdge < G :: EdgeRef > , G : IntoEdgesDirected , { type Item = G :: NodeId ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; let from = self . from ; (& mut self . iter) . filter_map (move | edge | { if f . include_edge (edge) { if edge . source () != from { Some (edge . source ()) } else { Some (edge . target ()) } } else { None } }) . next () } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
    };
}

impl_157!()