// Generated macro for impl_1699 (impl)
macro_rules! Depcrate_query_dsl_save_changes_dslimpl_1699 {
() => {
// Module: crate::query_dsl::save_changes_dsl
// Provides: {"impl_1699"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl < 'b , Changes , Output > UpdateAndFetchResults < Changes , Output > for SqliteConnection where Changes : Copy + Identifiable , Changes : AsChangeset < Target = < Changes as HasTable > :: Table > + IntoUpdateTarget , Changes :: Table : FindDsl < Changes :: Id > , Update < Changes , Changes > : ExecuteDsl < SqliteConnection > , Find < Changes :: Table , Changes :: Id > : LoadQuery < 'b , SqliteConnection , Output > , < Changes :: Table as Table > :: AllColumns : ValidGrouping < () > , < < Changes :: Table as Table > :: AllColumns as ValidGrouping < () > > :: IsAggregate : MixedAggregates < is_aggregate :: No , Output = is_aggregate :: No > , { fn update_and_fetch (& mut self , changeset : Changes) -> QueryResult < Output > { crate :: update (changeset) . set (changeset) . execute (self) ? ; Changes :: table () . find (changeset . id ()) . get_result (self) } }
};
}
