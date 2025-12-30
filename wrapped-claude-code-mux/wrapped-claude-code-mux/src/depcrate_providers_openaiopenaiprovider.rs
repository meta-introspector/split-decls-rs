// Generated macro for OpenAIProvider (struct)
macro_rules! Depcrate_providers_openaiOpenAIProvider {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIProvider"}
// Dependencies: {}
# [doc = " OpenAI provider implementation"] pub struct OpenAIProvider { name : String , api_key : String , base_url : String , client : Client , models : Vec < String > , custom_headers : Vec < (String , String) > , # [doc = " OAuth provider ID (if using OAuth instead of API key)"] oauth_provider : Option < String > , # [doc = " Token store for OAuth authentication"] token_store : Option < TokenStore > , }
};
}
