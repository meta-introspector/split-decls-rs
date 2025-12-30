// Generated macro for impl_1697 (impl)
macro_rules! Depcrate_query_dsl_save_changes_dslimpl_1697 {
() => {
// Module: crate::query_dsl::save_changes_dsl
// Provides: {"impl_1697"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl < 'b , Changes , Output > UpdateAndFetchResults < Changes , Output > for PgConnection where Changes : Copy + AsChangeset < Target = < Changes as HasTable > :: Table > + IntoUpdateTarget , Update < Changes , Changes > : LoadQuery < 'b , PgConnection , Output > , < Changes :: Table as Table > :: AllColumns : ValidGrouping < () > , < < Changes :: Table as Table > :: AllColumns as ValidGrouping < () > > :: IsAggregate : MixedAggregates < is_aggregate :: No , Output = is_aggregate :: No > , { fn update_and_fetch (& mut self , changeset : Changes) -> QueryResult < Output > { crate :: update (changeset) . set (changeset) . get_result (self) } }
};
}
