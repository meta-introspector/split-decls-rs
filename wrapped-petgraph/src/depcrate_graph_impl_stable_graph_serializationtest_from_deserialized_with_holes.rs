// Generated macro for test_from_deserialized_with_holes (function)
macro_rules! Depcrate_graph_impl_stable_graph_serializationtest_from_deserialized_with_holes {
() => {
// Module: crate::graph_impl::stable_graph::serialization
// Provides: {"test_from_deserialized_with_holes"}
// Dependencies: {}
# [test] fn test_from_deserialized_with_holes () { use alloc :: vec ; use itertools :: assert_equal ; use serde :: de :: value :: Error as SerdeError ; use crate :: graph :: node_index ; use crate :: stable_graph :: StableUnGraph ; let input = DeserStableGraph :: < _ , () , u32 > { nodes : vec ! [Node { weight : Some (1) , next : [EdgeIndex :: end () ; 2] , } , Node { weight : Some (4) , next : [EdgeIndex :: end () ; 2] , } , Node { weight : Some (5) , next : [EdgeIndex :: end () ; 2] , } ,] , node_holes : vec ! [node_index (0) , node_index (2) , node_index (3) , node_index (6)] , edges : vec ! [] , edge_property : EdgeProperty :: Undirected , } ; let graph = StableUnGraph :: from_deserialized :: < SerdeError > (input) . unwrap () ; assert_eq ! (graph . node_count () , 3) ; assert_equal (graph . raw_nodes () . iter () . map (| n | n . weight . as_ref () . cloned ()) , vec ! [None , Some (1) , None , None , Some (4) , Some (5) , None] ,) ; }
};
}
