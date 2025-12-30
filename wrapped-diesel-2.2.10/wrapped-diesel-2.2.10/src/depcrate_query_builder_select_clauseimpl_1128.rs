// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1128 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1128"}
// Dependencies: {}
impl < T , QS > SelectClauseExpression < FromClause < QS > > for SelectClause < T > where QS : QuerySource , T : SelectableExpression < QS > , { type Selection = T ; type SelectClauseSqlType = T :: SqlType ; }
};
}
