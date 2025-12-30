// Generated macro for ToolResultBlock (enum)
macro_rules! Depcrate_modelsToolResultBlock {
() => {
// Module: crate::models
// Provides: {"ToolResultBlock"}
// Dependencies: {}
# [doc = " Content blocks allowed in tool results"] # [derive (Debug , Clone , Deserialize , Serialize)] # [serde (tag = "type")] pub enum ToolResultBlock { # [serde (rename = "text")] Text { text : String } , # [serde (rename = "image")] Image { source : ImageSource } , }
};
}
