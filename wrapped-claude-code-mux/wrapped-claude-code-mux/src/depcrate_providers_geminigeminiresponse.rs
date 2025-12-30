// Generated macro for GeminiResponse (struct)
macro_rules! Depcrate_providers_geminiGeminiResponse {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiResponse"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (rename_all = "camelCase")] struct GeminiResponse { candidates : Vec < GeminiCandidate > , # [serde (skip_serializing_if = "Option::is_none")] usage_metadata : Option < GeminiUsageMetadata > , }
};
}
