macro_rules! deps {
    () => {
        Walker!();
        Bfs!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < G > Walker < G > for Bfs < G :: NodeId , G :: Map > where G : IntoNeighbors + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
    };
}

impl_61!()