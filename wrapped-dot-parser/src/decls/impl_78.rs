macro_rules! deps {
    () => {
        Graph!();
        Subgraph!();
        NodeID!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < A > Subgraph < A > { fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> Subgraph < B > { Subgraph { id : self . id , stmts : self . stmts . into_iter () . map (| stmt | stmt . filter_map_attr (f)) . collect () , } } # [doc = " Extract a subgraph as a standalone graph."] pub (crate) fn into_graph (self , strict : bool , is_digraph : bool) -> Graph < A > { Graph { strict , is_digraph , name : self . id . map (String :: from) , stmts : self . stmts , } } fn get_node_ids (& self) -> HashSet < NodeID > { self . stmts . get_node_ids () } }
    };
}

impl_78!()