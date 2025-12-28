macro_rules! deps {
    () => {
        NodeID!();
        Graph!();
        StmtList!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < A > Graph < A > { # [doc = " Filter and map attributes. The main intended usage of this function is"] # [doc = " to convert attributes as `&'a str` into an enum, e.g."] # [doc = " to convert `[\"label\"=\"whatever\", \"color\"=\"foo\"]` into"] # [doc = " `[Attr::Label(whatever), Attr::Color(foo)]`."] # [doc = ""] # [doc = " To take into account non-standard attributes, the `Attr` enum has to be"] # [doc = " provided by the user."] pub fn filter_map < B > (self , f : & dyn Fn (A) -> Option < B >) -> Graph < B > { let new_stmts : StmtList < B > = self . stmts . filter_map_attr (f) ; Graph { strict : self . strict , is_digraph : self . is_digraph , name : self . name , stmts : new_stmts , } } # [doc = " Returns all `NodeID`s that appear in the graph."] pub fn get_node_ids (self) -> HashSet < NodeID > { let clone = self . stmts ; clone . get_node_ids () } }
    };
}

impl_18!()