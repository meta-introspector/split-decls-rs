macro_rules! deps {
    () => {
        NodeID!();
        EdgeStmt!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < A > EdgeStmt < A > { fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> EdgeStmt < B > { EdgeStmt { from : self . from , next : self . next , attr : self . attr . map (| a | a . filter_map_attr (f)) , } } fn get_node_ids (& self) -> HashSet < NodeID > { let mut nexts = self . next . get_node_ids () ; nexts . insert (self . from . clone ()) ; nexts } }
    };
}

impl_129!()