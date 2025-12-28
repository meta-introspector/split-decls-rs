macro_rules! deps {
    () => {
        StmtList!();
        NodeID!();
        Graph!();
        EdgeStmt!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < A > Graph < A > { # [doc = " Filter and map attributes. The main intended usage of this function is"] # [doc = " to convert attributes as `&'a str` into an enum, e.g."] # [doc = " to convert `[\"label\"=\"whatever\", \"color\"=\"foo\"]` into"] # [doc = " `[Attr::Label(whatever), Attr::Color(foo)]`."] # [doc = ""] # [doc = " To take into account non-standard attributes, the `Attr` enum has to be"] # [doc = " provided by the user."] pub fn filter_map < B > (self , f : & dyn Fn (A) -> Option < B >) -> Graph < B > { let new_stmts : StmtList < B > = self . stmts . filter_map_attr (f) ; Graph { strict : self . strict , is_digraph : self . is_digraph , name : self . name , stmts : new_stmts , } } # [doc = " Returns all `NodeID`s that appear in the graph."] pub fn get_node_ids (& self) -> HashSet < NodeID > { self . stmts . get_node_ids () } # [doc = " Returns all [EdgeStmt]s that appear in the graph."] # [doc = " Notice that it does not recurse into nested subgraphs."] pub fn get_edges_stmts (self) -> Vec < EdgeStmt < A > > { self . stmts . get_edges_stmts () } }
    };
}

impl_118!()