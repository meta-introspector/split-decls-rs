// Generated macro for CodeAssistInnerRequest (struct)
macro_rules! Depcrate_providers_geminiCodeAssistInnerRequest {
() => {
// Module: crate::providers::gemini
// Provides: {"CodeAssistInnerRequest"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize)] # [serde (rename_all = "camelCase")] struct CodeAssistInnerRequest { contents : Vec < GeminiContent > , # [serde (skip_serializing_if = "Option::is_none")] system_instruction : Option < GeminiSystemInstruction > , # [serde (skip_serializing_if = "Option::is_none")] generation_config : Option < GeminiGenerationConfig > , # [serde (skip_serializing_if = "Option::is_none")] tools : Option < Vec < GeminiTool > > , # [serde (skip_serializing_if = "Option::is_none")] session_id : Option < String > , }
};
}
