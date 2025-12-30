// Generated macro for push_colon2 (function)
macro_rules! Depcrate_tokens_runtimepush_colon2 {
() => {
// Module: crate::tokens::runtime
// Provides: {"push_colon2"}
// Dependencies: {}
pub fn push_colon2 (tokens : & mut TokenStream) { match tokens . 0 . chars () . last () { Some (':') => tokens . push_str (" ::") , _ => tokens . push_str ("::") , } }
};
}
