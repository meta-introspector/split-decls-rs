// Generated macro for impl_808 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_808 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_808"}
// Dependencies: {}
impl < Combinator , Rule , Source , Rhs > Query for CombinationClause < Combinator , Rule , Source , Rhs > where Source : Query , Rhs : Query < SqlType = Source :: SqlType > , { type SqlType = Source :: SqlType ; }
};
}
