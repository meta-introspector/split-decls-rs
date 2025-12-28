macro_rules! deps {
    () => {
        NodeTrait!();
        EdgeType!();
        NeighborsDirected!();
    };
}

macro_rules! impl_895 {
    () => {
        deps!();
        impl < N , Ty > Iterator for NeighborsDirected < '_ , N , Ty > where N : NodeTrait , Ty : EdgeType , { type Item = N ; fn next (& mut self) -> Option < N > { if Ty :: is_directed () { let self_dir = self . dir ; let start_node = self . start_node ; (& mut self . iter) . filter_map (move | & (n , dir) | { if dir == self_dir || n == start_node { Some (n) } else { None } }) . next () } else { self . iter . next () . map (| & (n , _) | n) } } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . iter . size_hint () ; if Ty :: is_directed () { (0 , upper) } else { (lower , upper) } } }
    };
}

impl_895!();