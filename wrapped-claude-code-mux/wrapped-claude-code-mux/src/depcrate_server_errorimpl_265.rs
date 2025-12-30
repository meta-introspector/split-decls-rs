// Generated macro for impl_265 (impl)
macro_rules! Depcrate_server_errorimpl_265 {
() => {
// Module: crate::server::error
// Provides: {"impl_265"}
// Dependencies: {}
impl IntoResponse for AppError { fn into_response (self) -> Response { let (status , message) = match self { AppError :: RoutingError (msg) => (StatusCode :: BAD_REQUEST , msg) , AppError :: ParseError (msg) => (StatusCode :: INTERNAL_SERVER_ERROR , msg) , AppError :: ProviderError (msg) => (StatusCode :: BAD_GATEWAY , msg) , } ; let body = Json (serde_json :: json ! ({ "error" : { "type" : "error" , "message" : message } })) ; (status , body) . into_response () } }
};
}
