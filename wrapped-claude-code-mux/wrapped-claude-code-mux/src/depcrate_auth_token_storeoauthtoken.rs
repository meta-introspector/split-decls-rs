// Generated macro for OAuthToken (struct)
macro_rules! Depcrate_auth_token_storeOAuthToken {
() => {
// Module: crate::auth::token_store
// Provides: {"OAuthToken"}
// Dependencies: {}
# [doc = " OAuth token information"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OAuthToken { # [doc = " Provider ID (e.g., \"claude-max\", \"anthropic-oauth\")"] pub provider_id : String , # [doc = " OAuth access token"] pub access_token : String , # [doc = " OAuth refresh token"] pub refresh_token : String , # [doc = " Token expiration time (UTC)"] pub expires_at : DateTime < Utc > , # [doc = " Optional enterprise URL for GitHub Copilot Enterprise"] # [serde (skip_serializing_if = "Option::is_none")] pub enterprise_url : Option < String > , # [doc = " Optional Google Cloud project ID for Gemini Code Assist API"] # [serde (skip_serializing_if = "Option::is_none")] pub project_id : Option < String > , }
};
}
