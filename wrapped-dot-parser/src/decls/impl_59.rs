macro_rules! deps {
    () => {
        EdgeRHS!();
        NodeID!();
        EdgeStmt!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'a , A > EdgeStmt < A > { fn filter_map_attr < B > (self , f : & 'a dyn Fn (A) -> Option < B >) -> EdgeStmt < B > { let new_from = match self . from { Either :: Left (node_id) => Either :: Left (node_id) , Either :: Right (subgraph) => Either :: Right (subgraph . filter_map_attr (f)) , } ; let new_next = self . next . filter_map_attr (f) ; EdgeStmt { from : new_from , next : new_next , attr : self . attr . map (| a | a . filter_map_attr (f)) , } } # [doc = " Flatten the EdgeStmt, i.e. removes cases where multiple EdgeRHS are in a single statement."] pub fn flatten (self) -> Vec < EdgeStmt < A > > where A : Clone , { let mut from = self . from ; let mut to = self . next ; let attr = self . attr ; let mut v = Vec :: new () ; loop { let next_step = EdgeStmt { from : from . clone () , next : EdgeRHS { to : to . to . clone () , next : None , } , attr : attr . clone () , } ; v . push (next_step) ; match to . next { None => return v , Some (rhs) => { from = to . to ; to = * rhs ; } } } } fn get_node_ids (& self) -> HashSet < NodeID > { let mut nexts = self . next . get_node_ids () ; match & self . from { Either :: Left (node_id) => { nexts . insert (node_id . clone ()) ; } Either :: Right (subgraph) => { return nexts . union (& subgraph . get_node_ids ()) . cloned () . collect () ; } } ; nexts } }
    };
}

impl_59!();