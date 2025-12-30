// Generated macro for impl_1283 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1283 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1283"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC > SelectQuery for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where S : SelectClauseExpression < F > , O : ValidOrderingForDistinct < D > , { type SqlType = S :: SelectClauseSqlType ; }
};
}
