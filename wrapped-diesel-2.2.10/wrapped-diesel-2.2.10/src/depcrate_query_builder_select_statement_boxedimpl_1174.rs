// Generated macro for impl_1174 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1174 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1174"}
// Dependencies: {}
impl < 'a , ST , QS , DB , Predicate , GB > OrFilterDsl < Predicate > for BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > where QS : QuerySource , BoxedWhereClause < 'a , DB > : WhereOr < Predicate , Output = BoxedWhereClause < 'a , DB > > , Predicate : AppearsOnTable < QS > + NonAggregate , Predicate :: SqlType : BoolOrNullableBool , { type Output = Self ; fn or_filter (mut self , predicate : Predicate) -> Self :: Output { self . where_clause = self . where_clause . or (predicate) ; self } }
};
}
