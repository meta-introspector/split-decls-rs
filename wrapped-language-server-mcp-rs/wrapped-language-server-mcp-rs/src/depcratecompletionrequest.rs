// Generated macro for CompletionRequest (struct)
macro_rules! DepcrateCompletionRequest {
() => {
// Module: crate
// Provides: {"CompletionRequest"}
// Dependencies: {}
# [derive (Debug , serde :: Deserialize , schemars :: JsonSchema)] pub struct CompletionRequest { pub file_path : String , pub line : u32 , pub column : u32 , }
};
}
