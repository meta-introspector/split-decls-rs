// Generated macro for CodeAssistResponse (struct)
macro_rules! Depcrate_providers_geminiCodeAssistResponse {
() => {
// Module: crate::providers::gemini
// Provides: {"CodeAssistResponse"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (rename_all = "camelCase")] struct CodeAssistResponse { response : GeminiResponse , # [serde (skip_serializing_if = "Option::is_none")] trace_id : Option < String > , }
};
}
