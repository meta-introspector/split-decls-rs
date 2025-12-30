// Generated macro for impl_1183 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1183 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1183"}
// Dependencies: {}
impl < ST , QS , DB , T , GB > Insertable < T > for BoxedSelectStatement < '_ , ST , QS , DB , GB > where T : Table , Self : Query , < T :: AllColumns as ValidGrouping < () > > :: IsAggregate : MixedAggregates < is_aggregate :: No , Output = is_aggregate :: No > , { type Values = InsertFromSelect < Self , T :: AllColumns > ; fn values (self) -> Self :: Values { InsertFromSelect :: new (self) } }
};
}
