macro_rules! deps {
    () => {
        Graph!();
        EdgeRHS!();
        EdgeStmt!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < A > From < crate :: ast :: EdgeStmt < A > > for Vec < EdgeStmt < A > > where A : Clone , { fn from (edge : crate :: ast :: EdgeStmt < A >) -> Vec < EdgeStmt < A > > { let edges = edge . flatten () ; let mut v : Vec < EdgeStmt < A > > = Vec :: new () ; for edge in edges { let from = edge . from ; let to = edge . next . to ; let attr = edge . attr ; let (from_ids , mut extra_edges_from) = match from { Either :: Left (node_from) => (HashSet :: from_iter ([node_from]) , Vec :: new ()) , Either :: Right (subgraph) => { let g : Graph < A > = subgraph . into_graph (false , false) . into () ; (g . get_node_ids () , g . get_edges_stmts ()) } } ; let (to_ids , mut extra_edges_to) = match to { Either :: Left (node_from) => (HashSet :: from_iter ([node_from]) , Vec :: new ()) , Either :: Right (subgraph) => { let g : Graph < A > = subgraph . into_graph (false , false) . into () ; (g . get_node_ids () , g . get_edges_stmts ()) } } ; for from in from_ids { for to in & to_ids { v . push (EdgeStmt { from : from . clone () , next : EdgeRHS { to : to . clone () , next : None , } , attr : attr . clone () , }) ; } } v . append (& mut extra_edges_from) ; v . append (& mut extra_edges_to) ; } v } }
    };
}

impl_130!()