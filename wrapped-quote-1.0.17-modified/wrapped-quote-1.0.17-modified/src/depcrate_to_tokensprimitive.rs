// Generated macro for primitive (macro)
macro_rules! Depcrate_to_tokensprimitive {
() => {
// Module: crate::to_tokens
// Provides: {"primitive"}
// Dependencies: {}
macro_rules ! primitive { ($ ($ t : ident => $ name : ident) *) => { $ (impl ToTokens for $ t { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal ::$ name (* self)) ; } }) * } ; }
};
}
