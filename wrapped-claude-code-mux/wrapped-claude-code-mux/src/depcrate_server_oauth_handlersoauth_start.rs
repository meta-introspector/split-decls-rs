// Generated macro for oauth_start (function)
macro_rules! Depcrate_server_oauth_handlersoauth_start {
() => {
// Module: crate::server::oauth_handlers
// Provides: {"oauth_start"}
// Dependencies: {}
pub async fn oauth_start (Path (provider) : Path < String > , State (app_state) : State < Arc < AppState > > ,) -> Result < Redirect , AppError > { info ! ("OAuth start initiated for provider: {}" , provider) ; let config = app_state . config . read () . await . oauth . get (& provider) . cloned () . ok_or_else (| | AppError :: RoutingError (format ! ("OAuth provider {} not found" , provider))) ? ; let client = create_oauth_client (config , app_state . clone ()) . await ? ; let (authorize_url , csrf_state) = client . authorize_url (CsrfToken :: new_random) . add_scope (Scope :: new ("openid" . to_string ())) . add_scope (Scope :: new ("email" . to_string ())) . add_scope (Scope :: new ("profile" . to_string ())) . add_extra_param ("access_type" , "offline") . add_extra_param ("prompt" , "consent") . and_extra_query_param ("provider" , provider . clone ()) . url () ; app_state . token_store . save_csrf_token (provider , csrf_state . secret ()) . await ; Ok (Redirect :: to (authorize_url . as_str ())) }
};
}
