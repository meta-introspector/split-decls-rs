// Generated macro for impl_859 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_859 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_859"}
// Dependencies: {}
impl < T , U , Ret > QueryId for DeleteStatement < T , U , Ret > where T : QuerySource + QueryId + 'static , U : QueryId , Ret : QueryId , { type QueryId = DeleteStatement < T , U :: QueryId , Ret :: QueryId > ; const HAS_STATIC_QUERY_ID : bool = T :: HAS_STATIC_QUERY_ID && U :: HAS_STATIC_QUERY_ID && Ret :: HAS_STATIC_QUERY_ID ; }
};
}
