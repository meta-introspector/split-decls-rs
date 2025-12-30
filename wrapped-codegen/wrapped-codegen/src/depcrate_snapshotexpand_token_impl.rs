// Generated macro for expand_token_impl (function)
macro_rules! Depcrate_snapshotexpand_token_impl {
() => {
// Module: crate::snapshot
// Provides: {"expand_token_impl"}
// Dependencies: {}
fn expand_token_impl (name : & str , symbol : & str) -> TokenStream { let ident = Ident :: new (name , Span :: call_site ()) ; let repr = format ! ("Token![{}]" , symbol) ; quote ! { impl Debug for Lite < syn :: token ::# ident > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (# repr) } } } }
};
}
