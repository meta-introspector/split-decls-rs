// Generated macro for ProviderConfig (struct)
macro_rules! Depcrate_providersProviderConfig {
() => {
// Module: crate::providers
// Provides: {"ProviderConfig"}
// Dependencies: {}
# [doc = " Provider configuration from TOML"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProviderConfig { pub name : String , pub provider_type : String , # [doc = " Authentication type (default: api_key)"] # [serde (default)] pub auth_type : AuthType , # [doc = " API key (required for auth_type = \"apikey\")"] # [serde (skip_serializing_if = "Option::is_none")] pub api_key : Option < String > , # [doc = " OAuth provider ID (required for auth_type = \"oauth\")"] # [doc = " References a token stored in TokenStore"] # [serde (skip_serializing_if = "Option::is_none")] pub oauth_provider : Option < String > , # [doc = " Google Cloud Project ID (for Vertex AI provider)"] # [serde (skip_serializing_if = "Option::is_none")] pub project_id : Option < String > , # [doc = " Location/Region (for Vertex AI provider)"] # [serde (skip_serializing_if = "Option::is_none")] pub location : Option < String > , pub base_url : Option < String > , pub models : Vec < String > , pub enabled : Option < bool > , }
};
}
