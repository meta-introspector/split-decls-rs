// Generated macro for impl_38 (impl)
macro_rules! Depcrate_errorimpl_38 {
() => {
// Module: crate::error
// Provides: {"impl_38"}
// Dependencies: {}
impl ParsingToken { pub fn is_whitespace (& self , is_whitespace : & IsWhitespaceFn) -> bool { match self { ParsingToken :: Sensitive { token } => is_whitespace (token . clone ()) , ParsingToken :: Insensitive { token } => is_whitespace (token . clone ()) , ParsingToken :: Range { .. } => false , ParsingToken :: BuiltInRule => false , } } }
};
}
