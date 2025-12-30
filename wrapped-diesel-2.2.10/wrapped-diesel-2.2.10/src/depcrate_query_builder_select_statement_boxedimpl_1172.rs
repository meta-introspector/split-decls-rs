// Generated macro for impl_1172 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1172 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1172"}
// Dependencies: {}
impl < 'a , ST , QS , DB , Predicate , GB > FilterDsl < Predicate > for BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > where QS : QuerySource , BoxedWhereClause < 'a , DB > : WhereAnd < Predicate , Output = BoxedWhereClause < 'a , DB > > , Predicate : AppearsOnTable < QS > + NonAggregate , Predicate :: SqlType : BoolOrNullableBool , { type Output = Self ; fn filter (mut self , predicate : Predicate) -> Self :: Output { self . where_clause = self . where_clause . and (predicate) ; self } }
};
}
