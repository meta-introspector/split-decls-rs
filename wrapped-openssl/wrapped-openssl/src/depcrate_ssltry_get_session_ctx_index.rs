// Generated macro for try_get_session_ctx_index (function)
macro_rules! Depcrate_ssltry_get_session_ctx_index {
() => {
// Module: crate::ssl
// Provides: {"try_get_session_ctx_index"}
// Dependencies: {}
fn try_get_session_ctx_index () -> Result < & 'static Index < Ssl , SslContext > , ErrorStack > { SESSION_CTX_INDEX . get_or_try_init (Ssl :: new_ex_index) }
};
}
