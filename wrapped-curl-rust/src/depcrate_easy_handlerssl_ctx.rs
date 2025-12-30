// Generated macro for ssl_ctx (function)
macro_rules! Depcrate_easy_handlerssl_ctx {
() => {
// Module: crate::easy::handler
// Provides: {"ssl_ctx"}
// Dependencies: {}
pub fn ssl_ctx (cx : * mut c_void) -> Result < () , Error > { windows :: add_certs_to_context (cx) ; Ok (()) }
};
}
