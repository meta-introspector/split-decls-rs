// Generated macro for non_backtracking_dfs (function)
macro_rules! Depcrate_algo_matchingnon_backtracking_dfs {
() => {
// Module: crate::algo::matching
// Provides: {"non_backtracking_dfs"}
// Dependencies: {}
fn non_backtracking_dfs < G , F > (graph : & G , source : G :: NodeId , visited : & mut G :: Map , mut visitor : F) where G : Visitable + IntoNeighbors , F : FnMut (G :: NodeId) , { if visited . visit (source) { for target in graph . neighbors (source) { if ! visited . is_visited (& target) { visitor (target) ; non_backtracking_dfs (graph , target , visited , visitor) ; break ; } } } }
};
}
