// Generated macro for impl_1186 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1186 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1186"}
// Dependencies: {}
impl < 'a , ST , QS , DB , GB , Predicate > HavingDsl < Predicate > for BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > where QS : QuerySource , DB : Backend , GB : Expression , HavingClause < Predicate > : QueryFragment < DB > + Send + 'a , Predicate : AppearsOnTable < QS > , Predicate :: SqlType : BoolOrNullableBool , { type Output = Self ; fn having (mut self , predicate : Predicate) -> Self :: Output { self . having = Box :: new (HavingClause (predicate)) ; self } }
};
}
