macro_rules! deps {
    () => {
        Dfs!();
        Walker!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < G > Walker < G > for Dfs < G :: NodeId , G :: Map > where G : IntoNeighbors + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
    };
}

impl_59!();