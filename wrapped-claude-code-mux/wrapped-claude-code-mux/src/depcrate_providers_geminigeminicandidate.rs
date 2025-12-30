// Generated macro for GeminiCandidate (struct)
macro_rules! Depcrate_providers_geminiGeminiCandidate {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiCandidate"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (rename_all = "camelCase")] struct GeminiCandidate { content : GeminiContent , # [serde (skip_serializing_if = "Option::is_none")] finish_reason : Option < String > , }
};
}
