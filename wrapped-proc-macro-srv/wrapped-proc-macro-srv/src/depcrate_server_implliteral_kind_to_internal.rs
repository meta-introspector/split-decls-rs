// Generated macro for literal_kind_to_internal (function)
macro_rules! Depcrate_server_implliteral_kind_to_internal {
() => {
// Module: crate::server_impl
// Provides: {"literal_kind_to_internal"}
// Dependencies: {}
fn literal_kind_to_internal (kind : bridge :: LitKind) -> tt :: LitKind { match kind { bridge :: LitKind :: Byte => tt :: LitKind :: Byte , bridge :: LitKind :: Char => tt :: LitKind :: Char , bridge :: LitKind :: Str => tt :: LitKind :: Str , bridge :: LitKind :: StrRaw (r) => tt :: LitKind :: StrRaw (r) , bridge :: LitKind :: ByteStr => tt :: LitKind :: ByteStr , bridge :: LitKind :: ByteStrRaw (r) => tt :: LitKind :: ByteStrRaw (r) , bridge :: LitKind :: CStr => tt :: LitKind :: CStr , bridge :: LitKind :: CStrRaw (r) => tt :: LitKind :: CStrRaw (r) , bridge :: LitKind :: Integer => tt :: LitKind :: Integer , bridge :: LitKind :: Float => tt :: LitKind :: Float , bridge :: LitKind :: ErrWithGuar => tt :: LitKind :: Err (()) , } }
};
}
