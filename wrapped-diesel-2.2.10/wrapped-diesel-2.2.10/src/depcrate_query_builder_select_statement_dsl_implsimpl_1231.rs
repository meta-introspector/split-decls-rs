// Generated macro for impl_1231 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1231 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1231"}
// Dependencies: {}
impl < ST , F , S , D , W , O , LOf , G , H , LC , Expr > OrderDsl < Expr > for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H , LC > where F : QuerySource , Expr : AppearsOnTable < F > , Self : SelectQuery < SqlType = ST > , SelectStatement < FromClause < F > , S , D , W , OrderClause < Expr > , LOf , G , H , LC > : SelectQuery < SqlType = ST > , OrderClause < Expr > : ValidOrderingForDistinct < D > , { type Output = SelectStatement < FromClause < F > , S , D , W , OrderClause < Expr > , LOf , G , H , LC > ; fn order (self , expr : Expr) -> Self :: Output { let order = OrderClause (expr) ; SelectStatement :: new (self . select , self . from , self . distinct , self . where_clause , order , self . limit_offset , self . group_by , self . having , self . locking ,) } }
};
}
