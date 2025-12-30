// Generated macro for FindReferencesRequest (struct)
macro_rules! DepcrateFindReferencesRequest {
() => {
// Module: crate
// Provides: {"FindReferencesRequest"}
// Dependencies: {}
# [derive (Debug , serde :: Deserialize , schemars :: JsonSchema)] pub struct FindReferencesRequest { pub file_path : String , pub line : u32 , pub column : u32 , # [serde (default = "default_include_declaration")] pub include_declaration : bool , }
};
}
