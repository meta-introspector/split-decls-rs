// Generated macro for impl_1178 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1178 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1178"}
// Dependencies: {}
impl < 'a , ST , QS , DB , Order , GB > OrderDsl < Order > for BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > where DB : Backend , QS : QuerySource , Order : QueryFragment < DB > + AppearsOnTable < QS > + Send + 'a , { type Output = Self ; fn order (mut self , order : Order) -> Self :: Output { self . order = OrderClause (order) . into () ; self } }
};
}
