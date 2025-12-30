// Generated macro for impl_1656 (impl)
macro_rules! Depcrate_query_dsl_order_dslimpl_1656 {
() => {
// Module: crate::query_dsl::order_dsl
// Provides: {"impl_1656"}
// Dependencies: {}
impl < T , Expr > OrderDsl < Expr > for T where Expr : Expression , T : Table , T :: Query : OrderDsl < Expr > , { type Output = < T :: Query as OrderDsl < Expr > > :: Output ; fn order (self , expr : Expr) -> Self :: Output { self . as_query () . order (expr) } }
};
}
