macro_rules! deps {
    () => {
        EdgeRHS!();
        NodeID!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a , A > EdgeRHS < A > { fn filter_map_attr < B > (self , f : & 'a dyn Fn (A) -> Option < B >) -> EdgeRHS < B > { let to = match self . to { Either :: Left (node_id) => Either :: Left (node_id) , Either :: Right (subgraph) => Either :: Right (subgraph . filter_map_attr (f)) , } ; let next = self . next . map (| e | Box :: new (e . filter_map_attr (f))) ; EdgeRHS { to , next } } fn get_node_ids (& self) -> HashSet < NodeID > { let mut nexts : HashSet < NodeID > = self . next . as_ref () . map (| n | n . get_node_ids ()) . unwrap_or_default () ; match & self . to { Either :: Left (node_id) => { nexts . insert (node_id . clone ()) ; } Either :: Right (subgraph) => { return nexts . union (& subgraph . get_node_ids ()) . cloned () . collect () ; } } ; nexts } }
    };
}

impl_63!()