// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1175 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1175"}
// Dependencies: {}
impl < 'a , ST , DB , Predicate , GB > OrFilterDsl < Predicate > for BoxedSelectStatement < 'a , ST , NoFromClause , DB , GB > where BoxedWhereClause < 'a , DB > : WhereOr < Predicate , Output = BoxedWhereClause < 'a , DB > > , Predicate : AppearsOnTable < NoFromClause > + NonAggregate , Predicate :: SqlType : BoolOrNullableBool , { type Output = Self ; fn or_filter (mut self , predicate : Predicate) -> Self :: Output { self . where_clause = self . where_clause . or (predicate) ; self } }
};
}
