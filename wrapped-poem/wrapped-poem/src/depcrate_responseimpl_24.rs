// Generated macro for impl_24 (impl)
macro_rules! Depcrate_responseimpl_24 {
() => {
// Module: crate::response
// Provides: {"impl_24"}
// Dependencies: {}
impl IntoResponse for GraphQLResponse { fn into_response (self) -> Response { GraphQLBatchResponse (self . 0 . into ()) . into_response () } }
};
}
