// Generated macro for has_self_in_token_stream (function)
macro_rules! Depcrate_receiverhas_self_in_token_stream {
() => {
// Module: crate::receiver
// Provides: {"has_self_in_token_stream"}
// Dependencies: {}
fn has_self_in_token_stream (tokens : TokenStream) -> bool { tokens . into_iter () . any (| tt | match tt { TokenTree :: Ident (ident) => ident == "Self" , TokenTree :: Group (group) => has_self_in_token_stream (group . stream ()) , _ => false , }) }
};
}
