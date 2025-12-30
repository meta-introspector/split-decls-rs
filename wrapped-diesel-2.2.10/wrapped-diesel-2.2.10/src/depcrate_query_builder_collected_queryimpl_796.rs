// Generated macro for impl_796 (impl)
macro_rules! Depcrate_query_builder_collected_queryimpl_796 {
() => {
// Module: crate::query_builder::collected_query
// Provides: {"impl_796"}
// Dependencies: {}
impl < DB , T > QueryFragment < DB > for CollectedQuery < T > where DB : Backend + DieselReserveSpecialization , for < 'a > < DB as Backend > :: BindCollector < 'a > : MoveableBindCollector < DB , BindData = T > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { if ! self . safe_to_cache_prepared { pass . unsafe_to_cache_prepared () ; } pass . push_sql (& self . sql) ; pass . push_bind_collector_data :: < T > (& self . bind_data) } }
};
}
