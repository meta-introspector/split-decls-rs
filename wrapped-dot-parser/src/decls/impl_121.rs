macro_rules! deps {
    () => {
        EdgeStmt!();
        NodeID!();
        StmtList!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a , A > StmtList < A > { fn filter_map_attr < B > (self , f : & 'a dyn Fn (A) -> Option < B >) -> StmtList < B > { self . stmts . into_iter () . map (| stmt | stmt . filter_map_attr (f)) . collect () } fn get_node_ids (& self) -> HashSet < NodeID > { let mut hs = HashSet :: new () ; for stmt in self { hs = hs . union (& stmt . get_node_ids ()) . cloned () . collect () ; } hs } # [doc = " Returns a clone of all the EdgeStmt contained in the list."] fn get_edges_stmts (self) -> Vec < EdgeStmt < A > > { let mut v = Vec :: new () ; for stmt in self { if let Some (edge) = stmt . get_edge () { v . push (edge) } } v } }
    };
}

impl_121!();