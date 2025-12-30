// Generated macro for use_355 (use)
macro_rules! Depcrate_serveruse_355 {
() => {
// Module: crate::server
// Provides: {"use_355"}
// Dependencies: {}
use self :: { config_update :: ConfigUpdate , error :: AppError , handlers :: { get_config_json , get_models , get_models_config , get_providers , health_check , serve_admin , update_config , update_config_json , handle_openai_chat_completions , } , oauth_handlers :: { oauth_callback , oauth_login , oauth_logout , oauth_start } , openai_compat :: { open_ai_compat_completions , open_ai_compat_models , } , state :: { AppState , LogState } , utils :: create_and_execute_restart_script , } ;
};
}
