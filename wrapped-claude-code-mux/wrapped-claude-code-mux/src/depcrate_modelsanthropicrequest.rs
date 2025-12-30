// Generated macro for AnthropicRequest (struct)
macro_rules! Depcrate_modelsAnthropicRequest {
() => {
// Module: crate::models
// Provides: {"AnthropicRequest"}
// Dependencies: {}
# [doc = " Anthropic API request format"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct AnthropicRequest { pub model : String , pub messages : Vec < Message > , pub max_tokens : u32 , # [serde (skip_serializing_if = "Option::is_none")] pub thinking : Option < ThinkingConfig > , # [serde (skip_serializing_if = "Option::is_none")] pub temperature : Option < f32 > , # [serde (skip_serializing_if = "Option::is_none")] pub top_p : Option < f32 > , # [serde (skip_serializing_if = "Option::is_none")] pub top_k : Option < u32 > , # [serde (skip_serializing_if = "Option::is_none")] pub stop_sequences : Option < Vec < String > > , # [serde (skip_serializing_if = "Option::is_none")] pub stream : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] pub metadata : Option < HashMap < String , serde_json :: Value > > , # [serde (skip_serializing_if = "Option::is_none")] pub system : Option < SystemPrompt > , # [serde (skip_serializing_if = "Option::is_none")] pub tools : Option < Vec < Tool > > , }
};
}
