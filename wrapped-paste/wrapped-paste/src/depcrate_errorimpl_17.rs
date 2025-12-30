// Generated macro for impl_17 (impl)
macro_rules! Depcrate_errorimpl_17 {
() => {
// Module: crate::error
// Provides: {"impl_17"}
// Dependencies: {}
impl Error { pub fn new (span : Span , msg : & str) -> Self { Self :: new2 (span , span , msg) } pub fn new2 (begin : Span , end : Span , msg : & str) -> Self { Error { begin , end , msg : msg . to_owned () , } } pub fn to_compile_error (& self) -> TokenStream { TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new ("compile_error" , self . begin)) , TokenTree :: Punct ({ let mut punct = Punct :: new ('!' , Spacing :: Alone) ; punct . set_span (self . begin) ; punct }) , TokenTree :: Group ({ let mut group = Group :: new (Delimiter :: Brace , { TokenStream :: from_iter (vec ! [TokenTree :: Literal ({ let mut string = Literal :: string (& self . msg) ; string . set_span (self . end) ; string })]) }) ; group . set_span (self . end) ; group }) ,]) } }
};
}
