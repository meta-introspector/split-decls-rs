macro_rules! deps {
    () => {
        Edge!();
        EdgesWalkerMut!();
        IndexType!();
        EdgeIndex!();
    };
}

macro_rules! impl_695 {
    () => {
        deps!();
        impl < E , Ix > EdgesWalkerMut < '_ , E , Ix > where Ix : IndexType , { fn next_edge (& mut self) -> Option < & mut Edge < E , Ix > > { self . next () . map (| t | t . 1) } fn next (& mut self) -> Option < (EdgeIndex < Ix > , & mut Edge < E , Ix >) > { let this_index = self . next ; let k = self . dir . index () ; match self . edges . get_mut (self . next . index ()) { None => None , Some (edge) => { self . next = edge . next [k] ; Some ((this_index , edge)) } } } }
    };
}

impl_695!()