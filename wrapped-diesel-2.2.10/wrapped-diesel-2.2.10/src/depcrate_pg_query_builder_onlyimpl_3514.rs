// Generated macro for impl_3514 (impl)
macro_rules! Depcrate_pg_query_builder_onlyimpl_3514 {
() => {
// Module: crate::pg::query_builder::only
// Provides: {"impl_3514"}
// Dependencies: {}
impl < S > QueryId for Only < S > where Self : 'static , S : QueryId , { type QueryId = Self ; const HAS_STATIC_QUERY_ID : bool = < S as QueryId > :: HAS_STATIC_QUERY_ID ; }
};
}
