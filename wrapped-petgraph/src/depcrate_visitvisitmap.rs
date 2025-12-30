// Generated macro for VisitMap (trait)
macro_rules! Depcrate_visitVisitMap {
() => {
// Module: crate::visit
// Provides: {"VisitMap"}
// Dependencies: {}
# [doc = " A mapping for storing the visited status for NodeId `N`."] pub trait VisitMap < N > { # [doc = " Mark `a` as visited."] # [doc = ""] # [doc = " Return **true** if this is the first visit, false otherwise."] fn visit (& mut self , a : N) -> bool ; # [doc = " Return whether `a` has been visited before."] fn is_visited (& self , a : & N) -> bool ; # [doc = " Mark `a` as unvisited."] # [doc = ""] # [doc = " Return **true** if this vertex was marked as visited at the time of unsetting it, false otherwise."] fn unvisit (& mut self , _a : N) -> bool ; }
};
}
