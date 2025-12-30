// Generated macro for contains_fn (function)
macro_rules! Depcrate_receivercontains_fn {
() => {
// Module: crate::receiver
// Provides: {"contains_fn"}
// Dependencies: {}
fn contains_fn (tokens : TokenStream) -> bool { tokens . into_iter () . any (| tt | match tt { TokenTree :: Ident (ident) => ident == "fn" , TokenTree :: Group (group) => contains_fn (group . stream ()) , _ => false , }) }
};
}
