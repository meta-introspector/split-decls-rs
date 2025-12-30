// Generated macro for impl_1232 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1232 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1232"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Expr > ThenOrderDsl < Expr > for SelectStatement < FromClause < F > , S , D , W , OrderClause < O > , LOf , G , H , LC > where F : QuerySource , Expr : AppearsOnTable < F > , { type Output = SelectStatement < FromClause < F > , S , D , W , OrderClause < (O , Expr) > , LOf , G , H , LC > ; fn then_order_by (self , expr : Expr) -> Self :: Output { SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , OrderClause ((self . order . 0 , expr)) , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
