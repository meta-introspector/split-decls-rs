// Generated macro for impl_header (function)
macro_rules! Depcrateimpl_header {
() => {
// Module: crate
// Provides: {"impl_header"}
// Dependencies: {}
fn impl_header (ast : & syn :: DeriveInput) -> proc_macro2 :: TokenStream { let fns = match impl_fns (ast) { Ok (fns) => fns , Err (msg) => { return quote ! { compile_error ! (# msg) ; } . into () ; } } ; let decode = fns . decode ; let encode = fns . encode ; let ty = & ast . ident ; let hname = fns . name . unwrap_or_else (| | to_header_name (& ty . to_string ())) ; let hname_ident = Ident :: new (& hname , Span :: call_site ()) ; let dummy_const = Ident :: new (& format ! ("_IMPL_HEADER_FOR_{}" , hname) , Span :: call_site ()) ; let impl_block = quote ! { impl __hc :: Header for # ty { fn name () -> &'static __hc :: HeaderName { & __hc :: header ::# hname_ident } fn decode <'i , I > (values : & mut I) -> Result < Self , __hc :: Error > where I : Iterator < Item = &'i __hc :: HeaderValue >, { # decode } fn encode < E : Extend < __hc :: HeaderValue >> (& self , values : & mut E) { # encode } } } ; quote ! { const # dummy_const : () = { extern crate headers_core as __hc ; # impl_block } ; } }
};
}
