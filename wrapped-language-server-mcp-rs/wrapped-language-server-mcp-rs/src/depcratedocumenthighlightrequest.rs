// Generated macro for DocumentHighlightRequest (struct)
macro_rules! DepcrateDocumentHighlightRequest {
() => {
// Module: crate
// Provides: {"DocumentHighlightRequest"}
// Dependencies: {}
# [derive (Debug , serde :: Deserialize , schemars :: JsonSchema)] pub struct DocumentHighlightRequest { pub file_path : String , pub line : u32 , pub column : u32 , }
};
}
