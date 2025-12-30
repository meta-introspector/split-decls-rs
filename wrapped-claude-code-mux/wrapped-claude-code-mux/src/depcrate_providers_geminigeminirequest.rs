// Generated macro for GeminiRequest (struct)
macro_rules! Depcrate_providers_geminiGeminiRequest {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiRequest"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize)] # [serde (rename_all = "camelCase")] struct GeminiRequest { contents : Vec < GeminiContent > , # [serde (skip_serializing_if = "Option::is_none")] system_instruction : Option < GeminiSystemInstruction > , # [serde (skip_serializing_if = "Option::is_none")] generation_config : Option < GeminiGenerationConfig > , # [serde (skip_serializing_if = "Option::is_none")] tools : Option < Vec < GeminiTool > > , }
};
}
