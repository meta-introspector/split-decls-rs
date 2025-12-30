// Generated macro for push_ident (function)
macro_rules! Depcrate_tokens_runtimepush_ident {
() => {
// Module: crate::tokens::runtime
// Provides: {"push_ident"}
// Dependencies: {}
pub fn push_ident (tokens : & mut TokenStream , s : & str) { match tokens . 0 . chars () . last () { None | Some (':') => { } _ => tokens . 0 . push (' ') , } tokens . push_str (s) ; }
};
}
