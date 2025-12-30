// Generated macro for MessageContent (enum)
macro_rules! Depcrate_modelsMessageContent {
() => {
// Module: crate::models
// Provides: {"MessageContent"}
// Dependencies: {}
# [doc = " Message content can be string or array of content blocks"] # [derive (Debug , Clone , Deserialize , Serialize)] # [serde (untagged)] pub enum MessageContent { Text (String) , Blocks (Vec < ContentBlock >) , }
};
}
