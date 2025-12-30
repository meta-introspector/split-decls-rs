// Generated macro for AnthropicCompatibleProvider (struct)
macro_rules! Depcrate_providers_anthropic_compatibleAnthropicCompatibleProvider {
() => {
// Module: crate::providers::anthropic_compatible
// Provides: {"AnthropicCompatibleProvider"}
// Dependencies: {}
# [doc = " Generic Anthropic-compatible provider"] # [doc = " Works with: Anthropic, OpenRouter, z.ai, Minimax, etc."] # [doc = " Any provider that accepts Anthropic Messages API format"] pub struct AnthropicCompatibleProvider { name : String , api_key : String , base_url : String , client : Client , models : Vec < String > , # [doc = " Custom headers to add (e.g., \"HTTP-Referer\" for OpenRouter)"] custom_headers : Vec < (String , String) > , # [doc = " OAuth provider ID (if using OAuth instead of API key)"] oauth_provider : Option < String > , # [doc = " Token store for OAuth authentication"] token_store : Option < TokenStore > , }
};
}
