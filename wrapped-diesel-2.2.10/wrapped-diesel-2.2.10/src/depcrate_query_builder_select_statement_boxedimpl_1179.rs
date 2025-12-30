// Generated macro for impl_1179 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1179 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1179"}
// Dependencies: {}
impl < 'a , ST , QS , DB , Order , GB > ThenOrderDsl < Order > for BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > where DB : Backend + 'a , QS : QuerySource , Order : QueryFragment < DB > + AppearsOnTable < QS > + Send + 'a , { type Output = Self ; fn then_order_by (mut self , order : Order) -> Self :: Output { self . order = match self . order { Some (old) => Some (Box :: new ((old , order))) , None => Some (Box :: new (order)) , } ; self } }
};
}
