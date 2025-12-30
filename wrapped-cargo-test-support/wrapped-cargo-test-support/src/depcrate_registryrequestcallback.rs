// Generated macro for RequestCallback (type)
macro_rules! Depcrate_registryRequestCallback {
() => {
// Module: crate::registry
// Provides: {"RequestCallback"}
// Dependencies: {}
type RequestCallback = Box < dyn Send + Fn (& Request , & HttpServer) -> Response > ;
};
}
