// Generated macro for impl_1805 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1805 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1805"}
// Dependencies: {}
impl < S , Expr > ThenOrderDsl < Expr > for Alias < S > where Expr : Expression , Self : AsQuery , < Self as AsQuery > :: Query : ThenOrderDsl < Expr > , { type Output = < < Self as AsQuery > :: Query as ThenOrderDsl < Expr > > :: Output ; fn then_order_by (self , expr : Expr) -> Self :: Output { self . as_query () . then_order_by (expr) } }
};
}
