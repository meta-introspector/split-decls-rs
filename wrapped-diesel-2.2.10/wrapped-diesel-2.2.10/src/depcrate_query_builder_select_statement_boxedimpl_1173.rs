// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1173 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1173"}
// Dependencies: {}
impl < 'a , ST , DB , Predicate , GB > FilterDsl < Predicate > for BoxedSelectStatement < 'a , ST , NoFromClause , DB , GB > where BoxedWhereClause < 'a , DB > : WhereAnd < Predicate , Output = BoxedWhereClause < 'a , DB > > , Predicate : AppearsOnTable < NoFromClause > + NonAggregate , Predicate :: SqlType : BoolOrNullableBool , { type Output = Self ; fn filter (mut self , predicate : Predicate) -> Self :: Output { self . where_clause = self . where_clause . and (predicate) ; self } }
};
}
