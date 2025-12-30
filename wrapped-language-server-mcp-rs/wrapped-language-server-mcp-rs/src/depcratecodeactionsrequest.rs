// Generated macro for CodeActionsRequest (struct)
macro_rules! DepcrateCodeActionsRequest {
() => {
// Module: crate
// Provides: {"CodeActionsRequest"}
// Dependencies: {}
# [derive (Debug , serde :: Deserialize , schemars :: JsonSchema)] pub struct CodeActionsRequest { pub file_path : String , pub line : u32 , pub column : u32 , }
};
}
