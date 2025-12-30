// Generated macro for ContentBlock (enum)
macro_rules! Depcrate_modelsContentBlock {
() => {
// Module: crate::models
// Provides: {"ContentBlock"}
// Dependencies: {}
# [doc = " Content block for multimodal messages"] # [derive (Debug , Clone , Deserialize , Serialize)] # [serde (tag = "type")] pub enum ContentBlock { # [serde (rename = "text")] Text { text : String } , # [serde (rename = "image")] Image { source : ImageSource , } , # [serde (rename = "tool_use")] ToolUse { id : String , name : String , input : serde_json :: Value , } , # [serde (rename = "tool_result")] ToolResult { tool_use_id : String , content : ToolResultContent , } , # [serde (rename = "thinking")] Thinking { thinking : String , signature : String , } , }
};
}
