// Generated macro for impl_1171 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1171 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1171"}
// Dependencies: {}
impl < 'a , ST , DB , Selection , GB > SelectDsl < Selection > for BoxedSelectStatement < 'a , ST , NoFromClause , DB , GB > where DB : Backend , Selection : SelectableExpression < NoFromClause > + QueryFragment < DB > + ValidGrouping < GB > + Send + 'a , { type Output = BoxedSelectStatement < 'a , Selection :: SqlType , NoFromClause , DB , GB > ; fn select (self , selection : Selection) -> Self :: Output { BoxedSelectStatement { select : Box :: new (selection) , from : self . from , distinct : self . distinct , where_clause : self . where_clause , order : self . order , limit_offset : self . limit_offset , group_by : self . group_by , having : self . having , _marker : PhantomData , } } }
};
}
