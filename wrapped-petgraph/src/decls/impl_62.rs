macro_rules! deps {
    () => {
        Topo!();
        Walker!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < G > Walker < G > for Topo < G :: NodeId , G :: Map > where G : IntoNeighborsDirected + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
    };
}

impl_62!();