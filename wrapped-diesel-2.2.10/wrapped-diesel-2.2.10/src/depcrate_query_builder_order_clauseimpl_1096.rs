// Generated macro for impl_1096 (impl)
macro_rules! Depcrate_query_builder_order_clauseimpl_1096 {
() => {
// Module: crate::query_builder::order_clause
// Provides: {"impl_1096"}
// Dependencies: {}
impl < DB > From < NoOrderClause > for Option < Box < dyn QueryFragment < DB > + Send + '_ > > where DB : Backend , { fn from (_ : NoOrderClause) -> Self { None } }
};
}
