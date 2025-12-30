// Generated macro for push_group (function)
macro_rules! Depcrate_tokens_runtimepush_group {
() => {
// Module: crate::tokens::runtime
// Provides: {"push_group"}
// Dependencies: {}
pub fn push_group (tokens : & mut TokenStream , delimiter : Delimiter , inner : TokenStream) { tokens . push_space () ; tokens . push (delimiter . open ()) ; tokens . combine (& inner) ; tokens . push_space () ; tokens . push (delimiter . close ()) ; }
};
}
