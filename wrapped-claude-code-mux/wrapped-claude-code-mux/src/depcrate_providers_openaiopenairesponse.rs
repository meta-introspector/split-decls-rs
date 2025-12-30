// Generated macro for OpenAIResponse (struct)
macro_rules! Depcrate_providers_openaiOpenAIResponse {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIResponse"}
// Dependencies: {}
# [doc = " OpenAI Chat Completions response format"] # [derive (Debug , Deserialize)] struct OpenAIResponse { id : String , # [serde (default , rename = "object")] _object : String , model : String , choices : Vec < OpenAIChoice > , usage : OpenAIUsage , }
};
}
