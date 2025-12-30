// Generated macro for reachable_with_var_join (function)
macro_rules! Depcrate_testreachable_with_var_join {
() => {
// Module: crate::test
// Provides: {"reachable_with_var_join"}
// Dependencies: {}
# [doc = " The original way to use datafrog -- computes reachable nodes from a set of edges"] fn reachable_with_var_join (edges : & [(u32 , u32)]) -> Relation < (u32 , u32) > { let edges : Relation < _ > = edges . iter () . collect () ; let mut iteration = Iteration :: new () ; let edges_by_successor = iteration . variable :: < (u32 , u32) > ("edges_invert") ; edges_by_successor . extend (edges . iter () . map (| & (n1 , n2) | (n2 , n1))) ; let reachable = iteration . variable :: < (u32 , u32) > ("reachable") ; reachable . insert (edges) ; while iteration . changed () { reachable . from_join (& reachable , & edges_by_successor , | & _ , & n3 , & n1 | (n1 , n3)) ; } reachable . complete () }
};
}
