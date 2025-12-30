// Generated macro for impl_1283 (impl)
macro_rules! Depcrate_stream_try_stream_try_filter_mapimpl_1283 {
() => {
// Module: crate::stream::try_stream::try_filter_map
// Provides: {"impl_1283"}
// Dependencies: {}
impl < St , Fut , F > TryFilterMap < St , Fut , F > { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending : None } } delegate_access_inner ! (stream , St , ()) ; }
};
}
