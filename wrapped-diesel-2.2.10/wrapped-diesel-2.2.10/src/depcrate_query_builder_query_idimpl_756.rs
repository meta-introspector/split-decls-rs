// Generated macro for impl_756 (impl)
macro_rules! Depcrate_query_builder_query_idimpl_756 {
() => {
// Module: crate::query_builder::query_id
// Provides: {"impl_756"}
// Dependencies: {}
impl < T : QueryId + ? Sized > QueryId for & T { type QueryId = T :: QueryId ; const HAS_STATIC_QUERY_ID : bool = T :: HAS_STATIC_QUERY_ID ; }
};
}
