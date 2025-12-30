// Generated macro for GeminiGenerationConfig (struct)
macro_rules! Depcrate_providers_geminiGeminiGenerationConfig {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiGenerationConfig"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize)] # [serde (rename_all = "camelCase")] struct GeminiGenerationConfig { # [serde (skip_serializing_if = "Option::is_none")] temperature : Option < f32 > , # [serde (skip_serializing_if = "Option::is_none")] top_p : Option < f32 > , # [serde (skip_serializing_if = "Option::is_none")] top_k : Option < i32 > , # [serde (skip_serializing_if = "Option::is_none")] max_output_tokens : Option < i32 > , # [serde (skip_serializing_if = "Option::is_none")] stop_sequences : Option < Vec < String > > , }
};
}
