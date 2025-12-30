// Generated macro for impl_1472 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1472 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1472"}
// Dependencies: {}
impl < Predicate > WhereAnd < Predicate > for NoWhereClause where Predicate : Expression , Predicate :: SqlType : BoolOrNullableBool , { type Output = WhereClause < Predicate > ; fn and (self , predicate : Predicate) -> Self :: Output { WhereClause (predicate) } }
};
}
