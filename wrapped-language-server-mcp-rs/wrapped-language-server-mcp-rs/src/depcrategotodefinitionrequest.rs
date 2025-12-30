// Generated macro for GotoDefinitionRequest (struct)
macro_rules! DepcrateGotoDefinitionRequest {
() => {
// Module: crate
// Provides: {"GotoDefinitionRequest"}
// Dependencies: {}
# [derive (Debug , serde :: Deserialize , schemars :: JsonSchema)] pub struct GotoDefinitionRequest { pub file_path : String , pub line : u32 , pub column : u32 , }
};
}
