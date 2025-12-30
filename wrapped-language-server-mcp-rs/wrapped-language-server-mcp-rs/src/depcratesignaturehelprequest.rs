// Generated macro for SignatureHelpRequest (struct)
macro_rules! DepcrateSignatureHelpRequest {
() => {
// Module: crate
// Provides: {"SignatureHelpRequest"}
// Dependencies: {}
# [derive (Debug , serde :: Deserialize , schemars :: JsonSchema)] pub struct SignatureHelpRequest { pub file_path : String , pub line : u32 , pub column : u32 , }
};
}
