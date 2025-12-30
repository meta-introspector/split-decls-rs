// Generated macro for impl_1130 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1130 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1130"}
// Dependencies: {}
impl < QS > SelectClauseExpression < FromClause < QS > > for DefaultSelectClause < FromClause < QS > > where QS : QuerySource , { type Selection = QS :: DefaultSelection ; type SelectClauseSqlType = < Self :: Selection as Expression > :: SqlType ; }
};
}
