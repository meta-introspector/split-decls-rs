// Generated macro for SystemPrompt (enum)
macro_rules! Depcrate_modelsSystemPrompt {
() => {
// Module: crate::models
// Provides: {"SystemPrompt"}
// Dependencies: {}
# [doc = " System prompt can be string or array of system blocks"] # [derive (Debug , Clone , Deserialize , Serialize)] # [serde (untagged)] pub enum SystemPrompt { Text (String) , Blocks (Vec < SystemBlock >) , }
};
}
