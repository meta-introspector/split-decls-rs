// Generated macro for open_ai_compat_completions (function)
macro_rules! Depcrate_server_openai_compatopen_ai_compat_completions {
() => {
// Module: crate::server::openai_compat
// Provides: {"open_ai_compat_completions"}
// Dependencies: {}
pub async fn open_ai_compat_completions () -> Result < Response , AppError > { Ok ((StatusCode :: NOT_IMPLEMENTED , "The /v1/completions endpoint is not yet supported in this OpenAI compatibility layer. Please use /v1/chat/completions." ,) . into_response ()) }
};
}
