// Generated macro for impl_1658 (impl)
macro_rules! Depcrate_query_dsl_order_dslimpl_1658 {
() => {
// Module: crate::query_dsl::order_dsl
// Provides: {"impl_1658"}
// Dependencies: {}
impl < T , Expr > ThenOrderDsl < Expr > for T where Expr : Expression , T : Table , T :: Query : ThenOrderDsl < Expr > , { type Output = < T :: Query as ThenOrderDsl < Expr > > :: Output ; fn then_order_by (self , expr : Expr) -> Self :: Output { self . as_query () . then_order_by (expr) } }
};
}
