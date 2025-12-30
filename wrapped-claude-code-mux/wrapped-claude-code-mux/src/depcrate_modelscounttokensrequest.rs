// Generated macro for CountTokensRequest (struct)
macro_rules! Depcrate_modelsCountTokensRequest {
() => {
// Module: crate::models
// Provides: {"CountTokensRequest"}
// Dependencies: {}
# [doc = " Request for counting tokens"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct CountTokensRequest { pub model : String , pub messages : Vec < Message > , # [serde (skip_serializing_if = "Option::is_none")] pub system : Option < SystemPrompt > , # [serde (skip_serializing_if = "Option::is_none")] pub tools : Option < Vec < Tool > > , }
};
}
