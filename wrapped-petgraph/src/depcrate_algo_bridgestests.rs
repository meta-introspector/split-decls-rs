// Generated macro for tests (module)
macro_rules! Depcrate_algo_bridgestests {
() => {
// Module: crate::algo::bridges
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: graph :: EdgeReference ; use crate :: graph :: UnGraph ; use crate :: visit :: EdgeRef ; # [test] fn test_bridges () { let mut g = UnGraph :: < i8 , i8 > :: new_undirected () ; let bridge_nodes = | g : & _ | { bridges (g) . map (| e : EdgeReference < _ > | (e . source () , e . target ())) . collect :: < Vec < _ > > () } ; assert_eq ! (bridge_nodes (& g) , vec ! []) ; let n0 = g . add_node (0) ; assert_eq ! (bridge_nodes (& g) , vec ! []) ; let n1 = g . add_node (1) ; assert_eq ! (bridge_nodes (& g) , vec ! []) ; g . add_edge (n0 , n1 , 0) ; assert_eq ! (bridge_nodes (& g) , vec ! [(n0 , n1)]) ; let n2 = g . add_node (2) ; assert_eq ! (bridge_nodes (& g) , vec ! [(n0 , n1)]) ; g . add_edge (n2 , n1 , 1) ; assert_eq ! (bridge_nodes (& g) , vec ! [(n0 , n1) , (n2 , n1)]) ; g . add_edge (n0 , n2 , 2) ; assert_eq ! (bridge_nodes (& g) , vec ! []) ; let n3 = g . add_node (3) ; let n4 = g . add_node (4) ; g . add_edge (n2 , n3 , 3) ; g . add_edge (n3 , n4 , 4) ; assert_eq ! (bridge_nodes (& g) , vec ! [(n2 , n3) , (n3 , n4)]) ; g . add_edge (n3 , n0 , 5) ; assert_eq ! (bridge_nodes (& g) , vec ! [(n3 , n4)]) ; } }
};
}
