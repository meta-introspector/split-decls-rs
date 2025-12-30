// Generated macro for get_config_json (function)
macro_rules! Depcrate_server_handlersget_config_json {
() => {
// Module: crate::server::handlers
// Provides: {"get_config_json"}
// Dependencies: {}
# [doc = " Get full configuration as JSON (for admin UI)"] pub async fn get_config_json (State (state) : State < Arc < AppState > >) -> impl IntoResponse { let config = state . config . read () . await ; let mut json_config = get_base_config_json (& config) ; if let Some (obj) = json_config . as_object_mut () { obj . insert ("providers" . to_string () , serde_json :: to_value (& config . providers) . unwrap ()) ; obj . insert ("models" . to_string () , serde_json :: to_value (& config . models) . unwrap ()) ; } Json (json_config) }
};
}
