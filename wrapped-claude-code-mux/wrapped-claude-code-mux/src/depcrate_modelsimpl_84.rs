// Generated macro for impl_84 (impl)
macro_rules! Depcrate_modelsimpl_84 {
() => {
// Module: crate::models
// Provides: {"impl_84"}
// Dependencies: {}
impl ToolResultContent { # [doc = " Convert to string (for OpenAI compatibility)"] pub fn to_string (& self) -> String { match self { ToolResultContent :: Text (s) => s . clone () , ToolResultContent :: Blocks (blocks) => { blocks . iter () . filter_map (| block | match block { ToolResultBlock :: Text { text } => Some (text . clone ()) , ToolResultBlock :: Image { .. } => Some ("[Image]" . to_string ()) , }) . collect :: < Vec < _ > > () . join ("\n") } } } }
};
}
