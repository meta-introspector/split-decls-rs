macro_rules! deps {
    () => {
        Walker!();
        DfsPostOrder!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < G > Walker < G > for DfsPostOrder < G :: NodeId , G :: Map > where G : IntoNeighbors + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
    };
}

impl_60!();