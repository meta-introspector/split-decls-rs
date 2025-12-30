// Generated macro for impl_129 (impl)
macro_rules! Depcrate_connection_statement_cacheimpl_129 {
() => {
// Module: crate::connection::statement_cache
// Provides: {"impl_129"}
// Dependencies: {}
impl < T , DB > QueryFragmentForCachedStatement < DB > for T where DB : Backend , DB :: QueryBuilder : Default , T : QueryFragment < DB > , { fn construct_sql (& self , backend : & DB) -> QueryResult < String > { let mut query_builder = DB :: QueryBuilder :: default () ; self . to_sql (& mut query_builder , backend) ? ; Ok (query_builder . finish ()) } fn is_safe_to_cache_prepared (& self , backend : & DB) -> QueryResult < bool > { < T as QueryFragment < DB > > :: is_safe_to_cache_prepared (self , backend) } }
};
}
