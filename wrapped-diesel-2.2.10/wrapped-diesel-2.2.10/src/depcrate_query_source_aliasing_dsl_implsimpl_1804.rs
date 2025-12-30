// Generated macro for impl_1804 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1804 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1804"}
// Dependencies: {}
impl < S , Expr > OrderDsl < Expr > for Alias < S > where Expr : Expression , Self : AsQuery , < Self as AsQuery > :: Query : OrderDsl < Expr > , { type Output = < < Self as AsQuery > :: Query as OrderDsl < Expr > > :: Output ; fn order (self , expr : Expr) -> Self :: Output { self . as_query () . order (expr) } }
};
}
