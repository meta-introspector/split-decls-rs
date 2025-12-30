// Generated macro for impl_1129 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1129 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1129"}
// Dependencies: {}
impl < T > SelectClauseExpression < NoFromClause > for SelectClause < T > where T : SelectableExpression < NoFromClause > , { type Selection = T ; type SelectClauseSqlType = T :: SqlType ; }
};
}
