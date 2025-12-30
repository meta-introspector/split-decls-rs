// Generated macro for impl_109 (impl)
macro_rules! Depcrate_placeholdersimpl_109 {
() => {
// Module: crate::placeholders
// Provides: {"impl_109"}
// Dependencies: {}
impl PlaceholderExpander { pub fn add (& mut self , id : ast :: NodeId , mut fragment : AstFragment) { panic ! ("AstFragment.mut_visit_with called - functionality temporarily disabled") ; self . expanded_fragments . insert (id , fragment) ; } fn remove (& mut self , id : ast :: NodeId) -> AstFragment { self . expanded_fragments . remove (& id) . unwrap () } }
};
}
