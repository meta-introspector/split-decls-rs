// Generated macro for impl_542 (impl)
macro_rules! Depcrate_algo_matchingimpl_542 {
() => {
// Module: crate::algo::matching
// Provides: {"impl_542"}
// Dependencies: {}
impl < G : GraphBase > PartialEq for Label < G > { fn eq (& self , other : & Self) -> bool { match (self , other) { (Label :: None , Label :: None) => true , (Label :: Start , Label :: Start) => true , (Label :: Vertex (v1) , Label :: Vertex (v2)) => v1 == v2 , (Label :: Edge (e1 , _) , Label :: Edge (e2 , _)) => e1 == e2 , (Label :: Flag (e1) , Label :: Flag (e2)) => e1 == e2 , _ => false , } } }
};
}
