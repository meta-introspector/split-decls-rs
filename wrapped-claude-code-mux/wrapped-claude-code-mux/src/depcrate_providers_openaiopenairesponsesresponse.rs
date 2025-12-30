// Generated macro for OpenAIResponsesResponse (struct)
macro_rules! Depcrate_providers_openaiOpenAIResponsesResponse {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIResponsesResponse"}
// Dependencies: {}
# [doc = " OpenAI Responses API response format (for Codex models)"] # [derive (Debug , Deserialize)] struct OpenAIResponsesResponse { id : String , model : String , output : Vec < ResponsesOutput > , usage : ResponsesUsage , }
};
}
