// Generated macro for get_models (function)
macro_rules! Depcrate_server_handlersget_models {
() => {
// Module: crate::server::handlers
// Provides: {"get_models"}
// Dependencies: {}
# [doc = " REMOVED: This endpoint was for LiteLLM integration which has been removed."] # [doc = " Models are now managed through the provider registry and config."] pub async fn get_models (State (_state) : State < Arc < AppState > >) -> Result < Json < serde_json :: Value > , AppError > { Err (AppError :: ParseError ("This endpoint has been removed. Use /api/models-config instead." . to_string ())) }
};
}
