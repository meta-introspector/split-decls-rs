// Generated macro for impl_1861 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1861 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1861"}
// Dependencies: {}
impl < Left , Right , Kind > QueryId for Join < Left , Right , Kind > where Left : QueryId + QuerySource + 'static , Right : QueryId + QuerySource + 'static , Kind : QueryId , { type QueryId = Join < Left , Right , Kind :: QueryId > ; const HAS_STATIC_QUERY_ID : bool = Left :: HAS_STATIC_QUERY_ID && Right :: HAS_STATIC_QUERY_ID && Kind :: HAS_STATIC_QUERY_ID ; }
};
}
