// Generated macro for OAuthConfig (struct)
macro_rules! Depcrate_auth_oauthOAuthConfig {
() => {
// Module: crate::auth::oauth
// Provides: {"OAuthConfig"}
// Dependencies: {}
# [doc = " OAuth provider configuration"] # [derive (Debug , Clone)] pub struct OAuthConfig { pub client_id : String , pub client_secret : Option < String > , pub auth_url : String , pub token_url : String , pub redirect_uri : String , pub scopes : Vec < String > , }
};
}
