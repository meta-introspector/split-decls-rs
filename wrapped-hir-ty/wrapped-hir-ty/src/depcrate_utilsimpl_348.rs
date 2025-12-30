// Generated macro for impl_348 (impl)
macro_rules! Depcrate_utilsimpl_348 {
() => {
// Module: crate::utils
// Provides: {"impl_348"}
// Dependencies: {}
impl ClauseElaborator < '_ > { fn extend_deduped (& mut self , clauses : impl IntoIterator < Item = WhereClause >) { self . stack . extend (clauses . into_iter () . filter (| c | self . seen . insert (c . clone ()))) } fn elaborate_supertrait (& mut self , clause : & WhereClause) { if let WhereClause :: Implemented (trait_ref) = clause { direct_super_trait_refs (self . db , trait_ref , | t | { let clause = WhereClause :: Implemented (t) ; if self . seen . insert (clause . clone ()) { self . stack . push (clause) ; } }) ; } } }
};
}
