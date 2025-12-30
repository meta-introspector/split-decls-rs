// Generated macro for impl_18 (impl)
macro_rules! Depcrate_collections_vecimpl_18 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > concurrent_stream :: IntoConcurrentStream for Vec < T > { type Item = T ; type IntoConcurrentStream = IntoConcurrentStream < T > ; fn into_co_stream (self) -> Self :: IntoConcurrentStream { let stream = from_iter (self) ; let co_stream = stream . co () ; IntoConcurrentStream (co_stream) } }
};
}
