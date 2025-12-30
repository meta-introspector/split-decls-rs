// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Error { const fn new (span : Span , message : String) -> Self { Self { start : span , end : span , message , } } fn new_spanned < T > (tokens : T , message : & 'static str) -> Self where T : Into < TokenStream > , { let mut tokens = tokens . into () . into_iter () ; let start = tokens . next () . map (| x | x . span ()) . unwrap_or_else (Span :: call_site) ; Self { start , end : tokens . last () . map (| x | x . span ()) . unwrap_or (start) , message : message . to_owned () , } } fn into_compile_error (self) -> TokenStream { let mut result : TokenStream = macro_path ("std" , "compile_error") . map (| mut token | { token . set_span (self . start) ; token }) . collect () ; let mut literal = Literal :: string (& self . message) ; literal . set_span (self . end) ; let mut group = Group :: new (Delimiter :: Brace , TokenTree :: Literal (literal) . into ()) ; group . set_span (self . end) ; result . push (group) ; result } }
};
}
