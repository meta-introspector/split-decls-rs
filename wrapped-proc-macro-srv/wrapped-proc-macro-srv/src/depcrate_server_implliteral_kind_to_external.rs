// Generated macro for literal_kind_to_external (function)
macro_rules! Depcrate_server_implliteral_kind_to_external {
() => {
// Module: crate::server_impl
// Provides: {"literal_kind_to_external"}
// Dependencies: {}
fn literal_kind_to_external (kind : tt :: LitKind) -> bridge :: LitKind { match kind { tt :: LitKind :: Byte => bridge :: LitKind :: Byte , tt :: LitKind :: Char => bridge :: LitKind :: Char , tt :: LitKind :: Integer => bridge :: LitKind :: Integer , tt :: LitKind :: Float => bridge :: LitKind :: Float , tt :: LitKind :: Str => bridge :: LitKind :: Str , tt :: LitKind :: StrRaw (r) => bridge :: LitKind :: StrRaw (r) , tt :: LitKind :: ByteStr => bridge :: LitKind :: ByteStr , tt :: LitKind :: ByteStrRaw (r) => bridge :: LitKind :: ByteStrRaw (r) , tt :: LitKind :: CStr => bridge :: LitKind :: CStr , tt :: LitKind :: CStrRaw (r) => bridge :: LitKind :: CStrRaw (r) , tt :: LitKind :: Err (_) => bridge :: LitKind :: ErrWithGuar , } }
};
}
