// Generated macro for impl_3164 (impl)
macro_rules! Depcrate_pg_backendimpl_3164 {
() => {
// Module: crate::pg::backend
// Provides: {"impl_3164"}
// Dependencies: {}
impl FailedToLookupTypeError { # [doc = " Construct a new instance of this error type"] # [doc = " containing information about which type lookup failed"] # [cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub fn new (cache_key : PgMetadataCacheKey < 'static >) -> Self { Self :: new_internal (cache_key) } pub (in crate :: pg) fn new_internal (cache_key : PgMetadataCacheKey < 'static >) -> Self { Self (Box :: new (cache_key)) } }
};
}
