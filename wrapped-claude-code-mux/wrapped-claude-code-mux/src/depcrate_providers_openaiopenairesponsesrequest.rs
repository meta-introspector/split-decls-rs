// Generated macro for OpenAIResponsesRequest (struct)
macro_rules! Depcrate_providers_openaiOpenAIResponsesRequest {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIResponsesRequest"}
// Dependencies: {}
# [doc = " OpenAI Responses API request format (for Codex models)"] # [derive (Debug , Serialize)] struct OpenAIResponsesRequest { model : String , input : OpenAIResponsesInput , # [doc = " System instructions for the model (required for ChatGPT Codex)"] instructions : String , # [doc = " Whether to store the conversation (must be false for ChatGPT backend)"] store : bool , # [doc = " Enable streaming responses"] stream : bool , }
};
}
