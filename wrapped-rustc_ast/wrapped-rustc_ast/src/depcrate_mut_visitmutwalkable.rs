// Generated macro for MutWalkable (trait)
macro_rules! Depcrate_mut_visitMutWalkable {
() => {
// Module: crate::mut_visit
// Provides: {"MutWalkable"}
// Dependencies: {}
pub trait MutWalkable < V : MutVisitor > { fn walk_mut (& mut self , visitor : & mut V) ; }
};
}
