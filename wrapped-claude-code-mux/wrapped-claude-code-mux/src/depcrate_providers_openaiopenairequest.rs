// Generated macro for OpenAIRequest (struct)
macro_rules! Depcrate_providers_openaiOpenAIRequest {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIRequest"}
// Dependencies: {}
# [doc = " OpenAI Chat Completions request format"] # [derive (Debug , Serialize)] struct OpenAIRequest { model : String , messages : Vec < OpenAIMessage > , # [serde (skip_serializing_if = "Option::is_none")] max_tokens : Option < u32 > , # [serde (skip_serializing_if = "Option::is_none")] temperature : Option < f32 > , # [serde (skip_serializing_if = "Option::is_none")] top_p : Option < f32 > , # [serde (skip_serializing_if = "Option::is_none")] stop : Option < Vec < String > > , # [serde (skip_serializing_if = "Option::is_none")] stream : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] tools : Option < Vec < OpenAITool > > , # [serde (skip_serializing_if = "Option::is_none")] tool_choice : Option < serde_json :: Value > , }
};
}
