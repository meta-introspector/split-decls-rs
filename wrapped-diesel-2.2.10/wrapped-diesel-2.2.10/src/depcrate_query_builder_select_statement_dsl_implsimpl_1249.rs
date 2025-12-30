// Generated macro for impl_1249 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1249 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1249"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Tab > Insertable < Tab > for & SelectStatement < F , S , D , W , O , LOf , G , H , LC > where Tab : Table , Self : Query , < Tab :: AllColumns as ValidGrouping < () > > :: IsAggregate : MixedAggregates < is_aggregate :: No , Output = is_aggregate :: No > , { type Values = InsertFromSelect < Self , Tab :: AllColumns > ; fn values (self) -> Self :: Values { InsertFromSelect :: new (self) } }
};
}
