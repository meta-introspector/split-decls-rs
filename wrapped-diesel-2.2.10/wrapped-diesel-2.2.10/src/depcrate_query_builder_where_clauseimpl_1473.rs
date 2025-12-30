// Generated macro for impl_1473 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1473 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1473"}
// Dependencies: {}
impl < Predicate > WhereOr < Predicate > for NoWhereClause where Predicate : Expression , Predicate :: SqlType : BoolOrNullableBool , { type Output = WhereClause < Predicate > ; fn or (self , predicate : Predicate) -> Self :: Output { WhereClause (predicate) } }
};
}
