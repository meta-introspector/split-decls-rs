// Generated macro for MutVisitable (trait)
macro_rules! Depcrate_mut_visitMutVisitable {
() => {
// Module: crate::mut_visit
// Provides: {"MutVisitable"}
// Dependencies: {}
pub (crate) trait MutVisitable < V : MutVisitor > { type Extra : Copy ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) ; }
};
}
