// Generated macro for impl_541 (impl)
macro_rules! Depcrate_algo_matchingimpl_541 {
() => {
// Module: crate::algo::matching
// Provides: {"impl_541"}
// Dependencies: {}
impl < G : GraphBase > Label < G > { fn is_outer (& self) -> bool { self != & Label :: None && ! matches ! (self , Label :: Flag (_)) } fn is_inner (& self) -> bool { ! self . is_outer () } fn to_vertex (& self) -> Option < G :: NodeId > { match * self { Label :: Vertex (v) => Some (v) , _ => None , } } fn is_flagged (& self , edge : G :: EdgeId) -> bool { matches ! (self , Label :: Flag (flag) if flag == & edge) } }
};
}
