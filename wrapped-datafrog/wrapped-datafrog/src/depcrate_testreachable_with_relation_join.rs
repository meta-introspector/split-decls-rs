// Generated macro for reachable_with_relation_join (function)
macro_rules! Depcrate_testreachable_with_relation_join {
() => {
// Module: crate::test
// Provides: {"reachable_with_relation_join"}
// Dependencies: {}
# [doc = " Like `reachable`, but using a relation as an input to `from_join`"] fn reachable_with_relation_join (edges : & [(u32 , u32)]) -> Relation < (u32 , u32) > { let edges : Relation < _ > = edges . iter () . collect () ; let mut iteration = Iteration :: new () ; let edges_by_successor : Relation < _ > = edges . iter () . map (| & (n1 , n2) | (n2 , n1)) . collect () ; let reachable = iteration . variable :: < (u32 , u32) > ("reachable") ; reachable . insert (edges) ; while iteration . changed () { reachable . from_join (& reachable , & edges_by_successor , | & _ , & n3 , & n1 | (n1 , n3)) ; } reachable . complete () }
};
}
