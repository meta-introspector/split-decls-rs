// Generated macro for unsuffixed (macro)
macro_rules! Depcrate_tokens_token_streamunsuffixed {
() => {
// Module: crate::tokens::token_stream
// Provides: {"unsuffixed"}
// Dependencies: {}
macro_rules ! unsuffixed { ($ ty : ty => $ name : ident) => { pub fn $ name (n : $ ty) -> Self { Self { inner : n . to_string () , } } } ; }
};
}
