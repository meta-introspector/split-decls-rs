// Generated macro for impl_1312 (impl)
macro_rules! Depcrate_query_builder_sql_queryimpl_1312 {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"impl_1312"}
// Dependencies: {}
impl < Query , Value , ST > QueryId for UncheckedBind < Query , Value , ST > where Query : QueryId , ST : QueryId , { type QueryId = UncheckedBind < Query :: QueryId , () , ST :: QueryId > ; const HAS_STATIC_QUERY_ID : bool = Query :: HAS_STATIC_QUERY_ID && ST :: HAS_STATIC_QUERY_ID ; }
};
}
