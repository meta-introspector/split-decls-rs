// Generated macro for ToolResultContent (enum)
macro_rules! Depcrate_modelsToolResultContent {
() => {
// Module: crate::models
// Provides: {"ToolResultContent"}
// Dependencies: {}
# [doc = " Tool result content can be string or array of content blocks"] # [derive (Debug , Clone , Deserialize , Serialize)] # [serde (untagged)] pub enum ToolResultContent { Text (String) , Blocks (Vec < ToolResultBlock >) , }
};
}
