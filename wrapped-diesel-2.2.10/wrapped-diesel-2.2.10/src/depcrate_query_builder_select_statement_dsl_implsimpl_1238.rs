// Generated macro for impl_1238 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1238 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1238"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , Expr > GroupByDsl < Expr > for SelectStatement < F , S , D , W , O , LOf , G , H > where SelectStatement < F , S , D , W , O , LOf , GroupByClause < Expr > , H > : SelectQuery , Expr : Expression + AppearsOnTable < F > , { type Output = SelectStatement < F , S , D , W , O , LOf , GroupByClause < Expr > , H > ; fn group_by (self , expr : Expr) -> Self :: Output { let group_by = GroupByClause (expr) ; SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , self . order , self . limit_offset , group_by , self . having , self . locking ,) } }
};
}
