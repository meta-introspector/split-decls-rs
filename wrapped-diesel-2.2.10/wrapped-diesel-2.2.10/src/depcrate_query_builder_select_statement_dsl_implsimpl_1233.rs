// Generated macro for impl_1233 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1233 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1233"}
// Dependencies: {}
impl < F , S , D , W , LOf , G , LC , Expr > ThenOrderDsl < Expr > for SelectStatement < F , S , D , W , NoOrderClause , LOf , G , LC > where Expr : Expression , Self : OrderDsl < Expr > , { type Output = crate :: dsl :: Order < Self , Expr > ; fn then_order_by (self , expr : Expr) -> Self :: Output { self . order_by (expr) } }
};
}
