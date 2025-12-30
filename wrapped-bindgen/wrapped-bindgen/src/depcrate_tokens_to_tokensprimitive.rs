// Generated macro for primitive (macro)
macro_rules! Depcrate_tokens_to_tokensprimitive {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"primitive"}
// Dependencies: {}
macro_rules ! primitive { ($ ($ t : ident => $ name : ident) *) => ($ (impl ToTokens for $ t { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_space () ; tokens . push_str (& self . to_string ()) ; tokens . push_str (stringify ! ($ t)) ; } }) *) }
};
}
