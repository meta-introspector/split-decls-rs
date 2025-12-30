// Generated macro for impl_1170 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1170 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1170"}
// Dependencies: {}
impl < 'a , ST , QS , DB , Selection , GB > SelectDsl < Selection > for BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > where DB : Backend , QS : QuerySource , Selection : SelectableExpression < QS > + QueryFragment < DB > + ValidGrouping < GB > + Send + 'a , { type Output = BoxedSelectStatement < 'a , Selection :: SqlType , FromClause < QS > , DB , GB > ; fn select (self , selection : Selection) -> Self :: Output { BoxedSelectStatement { select : Box :: new (selection) , from : self . from , distinct : self . distinct , where_clause : self . where_clause , order : self . order , limit_offset : self . limit_offset , group_by : self . group_by , having : self . having , _marker : PhantomData , } } }
};
}
