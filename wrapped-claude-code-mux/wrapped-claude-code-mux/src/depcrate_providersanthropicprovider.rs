// Generated macro for AnthropicProvider (trait)
macro_rules! Depcrate_providersAnthropicProvider {
() => {
// Module: crate::providers
// Provides: {"AnthropicProvider"}
// Dependencies: {}
# [doc = " Main provider trait - all providers must implement this"] # [doc = " Maintains Anthropic Messages API compatibility"] # [async_trait] pub trait AnthropicProvider : Send + Sync { # [doc = " Send a message request to the provider"] # [doc = " Must transform to/from Anthropic format as needed"] async fn send_message (& self , request : AnthropicRequest) -> Result < ProviderResponse , ProviderError > ; # [doc = " Send a streaming message request to the provider"] # [doc = " Returns a stream of raw bytes (SSE format)"] async fn send_message_stream (& self , request : AnthropicRequest) -> Result < Pin < Box < dyn Stream < Item = Result < Bytes , ProviderError > > + Send > > , ProviderError > ; # [doc = " Count tokens for a request"] # [doc = " Provider-specific implementation (tiktoken for OpenAI, etc.)"] async fn count_tokens (& self , request : CountTokensRequest) -> Result < CountTokensResponse , ProviderError > ; # [doc = " Check if provider supports a specific model"] fn supports_model (& self , model : & str) -> bool ; }
};
}
