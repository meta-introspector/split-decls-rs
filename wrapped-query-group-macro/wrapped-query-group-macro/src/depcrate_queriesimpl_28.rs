// Generated macro for impl_28 (impl)
macro_rules! Depcrate_queriesimpl_28 {
() => {
// Module: crate::queries
// Provides: {"impl_28"}
// Dependencies: {}
impl ToTokens for Transparent { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & self . signature ; let ty = self . pat_and_tys . iter () . map (| pat_type | pat_type . pat . clone ()) . collect :: < Vec < Box < syn :: Pat > > > () ; let invoke = match & self . invoke { Some (path) => path . to_token_stream () , None => sig . ident . to_token_stream () , } ; let method = match & self . default { Some (default) => quote ! { # sig { let db = self ; # default } } , None => quote ! { # sig { # invoke (self , # (# ty) ,*) } } , } ; method . to_tokens (tokens) ; } }
};
}
