// Generated macro for open_ai_compat_models (function)
macro_rules! Depcrate_server_openai_compatopen_ai_compat_models {
() => {
// Module: crate::server::openai_compat
// Provides: {"open_ai_compat_models"}
// Dependencies: {}
pub async fn open_ai_compat_models () -> impl IntoResponse { Json (json ! ({ "object" : "list" , "data" : [{ "id" : "gpt-4" , "object" : "model" , "created" : 1677649551 , "owned_by" : "openai" , } , { "id" : "gpt-3.5-turbo" , "object" : "model" , "created" : 1677649551 , "owned_by" : "openai" , } ,] })) }
};
}
