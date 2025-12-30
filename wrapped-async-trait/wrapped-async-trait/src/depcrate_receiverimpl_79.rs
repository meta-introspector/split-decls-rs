// Generated macro for impl_79 (impl)
macro_rules! Depcrate_receiverimpl_79 {
() => {
// Module: crate::receiver
// Provides: {"impl_79"}
// Dependencies: {}
impl ReplaceSelf { fn visit_token_stream (& mut self , tokens : & mut TokenStream) -> bool { let mut out = Vec :: new () ; let mut modified = false ; visit_token_stream_impl (self , tokens . clone () , & mut modified , & mut out) ; if modified { * tokens = TokenStream :: from_iter (out) ; } return modified ; fn visit_token_stream_impl (visitor : & mut ReplaceSelf , tokens : TokenStream , modified : & mut bool , out : & mut Vec < TokenTree > ,) { for tt in tokens { match tt { TokenTree :: Ident (mut ident) => { * modified |= prepend_underscore_to_self (& mut ident) ; out . push (TokenTree :: Ident (ident)) ; } TokenTree :: Group (group) => { let mut content = group . stream () ; * modified |= visitor . visit_token_stream (& mut content) ; let mut new = Group :: new (group . delimiter () , content) ; new . set_span (group . span ()) ; out . push (TokenTree :: Group (new)) ; } other => out . push (other) , } } } } }
};
}
