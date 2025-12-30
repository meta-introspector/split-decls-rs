// Generated macro for impl_1282 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1282 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1282"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC > Query for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where G : ValidGroupByClause , S : SelectClauseExpression < F > , S :: Selection : ValidGrouping < G :: Expressions > , W : ValidWhereClause < F > , { type SqlType = S :: SelectClauseSqlType ; }
};
}
