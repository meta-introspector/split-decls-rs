// Generated macro for Visitable (trait)
macro_rules! Depcrate_visitorsVisitable {
() => {
// Module: crate::visitors
// Provides: {"Visitable"}
// Dependencies: {}
# [doc = " A type which can be visited."] pub trait Visitable < 'tcx > { # [doc = " Calls the corresponding `visit_*` function on the visitor."] fn visit < V : Visitor < 'tcx > > (self , visitor : & mut V) -> V :: Result ; }
};
}
