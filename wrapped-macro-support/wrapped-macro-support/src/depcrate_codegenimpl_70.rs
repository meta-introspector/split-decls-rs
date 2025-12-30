// Generated macro for impl_70 (impl)
macro_rules! Depcrate_codegenimpl_70 {
() => {
// Module: crate::codegen
// Provides: {"impl_70"}
// Dependencies: {}
impl TryToTokens for ast :: ImportKind { fn try_to_tokens (& self , tokens : & mut TokenStream) -> Result < () , Diagnostic > { match * self { ast :: ImportKind :: Function (ref f) => f . try_to_tokens (tokens) ? , ast :: ImportKind :: Static (ref s) => s . to_tokens (tokens) , ast :: ImportKind :: String (ref s) => s . to_tokens (tokens) , ast :: ImportKind :: Type (ref t) => t . to_tokens (tokens) , ast :: ImportKind :: Enum (ref e) => e . to_tokens (tokens) , } Ok (()) } }
};
}
