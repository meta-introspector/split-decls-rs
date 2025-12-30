// Generated macro for impl_1185 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1185 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1185"}
// Dependencies: {}
impl < 'a , ST , QS , DB , GB > SelectNullableDsl for BoxedSelectStatement < 'a , ST , QS , DB , GB > where ST : IntoNullable , { type Output = BoxedSelectStatement < 'a , ST :: Nullable , QS , DB > ; fn nullable (self) -> Self :: Output { BoxedSelectStatement { select : self . select , from : self . from , distinct : self . distinct , where_clause : self . where_clause , order : self . order , limit_offset : self . limit_offset , group_by : self . group_by , having : self . having , _marker : PhantomData , } } }
};
}
