// Generated macro for create_oauth_client (function)
macro_rules! Depcrate_server_oauth_handlerscreate_oauth_client {
() => {
// Module: crate::server::oauth_handlers
// Provides: {"create_oauth_client"}
// Dependencies: {}
async fn create_oauth_client (config : OAuthConfig , app_state : Arc < AppState >) -> Result < BasicClient , AppError > { let client_id = ClientId :: new (config . client_id) ; let client_secret = config . client_secret . map (ClientSecret :: new) ; let auth_url = AuthUrl :: new (config . auth_url) . map_err (| e | AppError :: ParseError (format ! ("Invalid AuthUrl: {}" , e))) ? ; let token_url = TokenUrl :: new (config . token_url) . map_err (| e | AppError :: ParseError (format ! ("Invalid TokenUrl: {}" , e))) ? ; let redirect_url = app_state . config . read () . await . server . public_url . join ("/oauth/callback") . map_err (| e | AppError :: ParseError (format ! ("Invalid redirect URL: {}" , e))) ? ; let redirect_url = RedirectUrl :: new (redirect_url . to_string ()) . map_err (| e | AppError :: ParseError (format ! ("Invalid RedirectUrl: {}" , e))) ? ; let client = BasicClient :: new (client_id , client_secret , auth_url , Some (token_url)) . set_redirect_uri (redirect_url) ; Ok (client) }
};
}
