// Generated macro for impl_809 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_809 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_809"}
// Dependencies: {}
impl < Combinator , Rule , Source , Rhs > SelectQuery for CombinationClause < Combinator , Rule , Source , Rhs > where Source : SelectQuery , Rhs : SelectQuery < SqlType = Source :: SqlType > , { type SqlType = Source :: SqlType ; }
};
}
