// Generated macro for impl_755 (impl)
macro_rules! Depcrate_query_builder_query_idimpl_755 {
() => {
// Module: crate::query_builder::query_id
// Provides: {"impl_755"}
// Dependencies: {}
impl < T : QueryId + ? Sized > QueryId for Box < T > { type QueryId = T :: QueryId ; const HAS_STATIC_QUERY_ID : bool = T :: HAS_STATIC_QUERY_ID ; }
};
}
