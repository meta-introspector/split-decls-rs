// Generated macro for GeminiPart (enum)
macro_rules! Depcrate_providers_geminiGeminiPart {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiPart"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] # [serde (untagged)] enum GeminiPart { Text { text : String } , InlineData { inline_data : GeminiInlineData } , }
};
}
