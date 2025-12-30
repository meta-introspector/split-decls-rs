// Generated macro for proj1 (function)
macro_rules! Depcrate_adjproj1 {
() => {
// Module: crate::adj
// Provides: {"proj1"}
// Dependencies: {}
fn proj1 < E , Ix : IndexType > (((successor_index , edge) , from) : ((usize , & WSuc < E , Ix >) , Ix) ,) -> EdgeReference < '_ , E , Ix > { let id = EdgeIndex { from , successor_index , } ; EdgeReference { id , edge } }
};
}
