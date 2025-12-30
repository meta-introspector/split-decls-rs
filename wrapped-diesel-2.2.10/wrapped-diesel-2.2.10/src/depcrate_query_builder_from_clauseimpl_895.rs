// Generated macro for impl_895 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_895 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_895"}
// Dependencies: {}
impl < F > Clone for FromClause < F > where F : QuerySource + Clone , F :: FromClause : Clone , { fn clone (& self) -> Self { Self { source : self . source . clone () , from_clause : self . from_clause . clone () , } } }
};
}
