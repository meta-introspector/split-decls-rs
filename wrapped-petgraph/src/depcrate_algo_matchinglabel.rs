// Generated macro for Label (enum)
macro_rules! Depcrate_algo_matchingLabel {
() => {
// Module: crate::algo::matching
// Provides: {"Label"}
// Dependencies: {}
# [derive (Clone , Copy , Default)] enum Label < G : GraphBase > { # [default] None , Start , Vertex (G :: NodeId) , Edge (G :: EdgeId , [G :: NodeId ; 2]) , Flag (G :: EdgeId) , }
};
}
