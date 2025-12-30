// Generated macro for RenameRequest (struct)
macro_rules! DepcrateRenameRequest {
() => {
// Module: crate
// Provides: {"RenameRequest"}
// Dependencies: {}
# [derive (Debug , serde :: Deserialize , schemars :: JsonSchema)] pub struct RenameRequest { pub file_path : String , pub line : u32 , pub column : u32 , pub new_name : String , }
};
}
