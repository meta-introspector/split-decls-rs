// Generated macro for reachable_with_leapfrog (function)
macro_rules! Depcrate_testreachable_with_leapfrog {
() => {
// Module: crate::test
// Provides: {"reachable_with_leapfrog"}
// Dependencies: {}
fn reachable_with_leapfrog (edges : & [(u32 , u32)]) -> Relation < (u32 , u32) > { let edges : Relation < _ > = edges . iter () . collect () ; let mut iteration = Iteration :: new () ; let edges_by_successor : Relation < _ > = edges . iter () . map (| & (n1 , n2) | (n2 , n1)) . collect () ; let reachable = iteration . variable :: < (u32 , u32) > ("reachable") ; reachable . insert (edges) ; while iteration . changed () { reachable . from_leapjoin (& reachable , edges_by_successor . extend_with (| & (n2 , _) | n2) , | & (_ , n3) , & n1 | (n1 , n3) ,) ; } reachable . complete () }
};
}
