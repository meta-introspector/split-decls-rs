// Generated macro for impl_1095 (impl)
macro_rules! Depcrate_query_builder_order_clauseimpl_1095 {
() => {
// Module: crate::query_builder::order_clause
// Provides: {"impl_1095"}
// Dependencies: {}
impl < 'a , DB , Expr > From < OrderClause < Expr > > for Option < Box < dyn QueryFragment < DB > + Send + 'a > > where DB : Backend , Expr : QueryFragment < DB > + Send + 'a , { fn from (order : OrderClause < Expr >) -> Self { Some (Box :: new (order . 0)) } }
};
}
