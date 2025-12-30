// Generated macro for impl_1021 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1021 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1021"}
// Dependencies: {}
impl < T , U , Op , Ret > QueryId for InsertStatement < T , U , Op , Ret > where T : QuerySource + QueryId + 'static , U : QueryId , Op : QueryId , Ret : QueryId , { type QueryId = InsertStatement < T , U :: QueryId , Op :: QueryId , Ret :: QueryId > ; const HAS_STATIC_QUERY_ID : bool = T :: HAS_STATIC_QUERY_ID && U :: HAS_STATIC_QUERY_ID && Op :: HAS_STATIC_QUERY_ID && Ret :: HAS_STATIC_QUERY_ID ; }
};
}
