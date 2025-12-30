// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl Error { pub (crate) fn new (span : Span , msg : String) -> Self { Self { span , msg } } pub (crate) fn into_compile_error (self) -> TokenStream { TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new ("compile_error" , self . span)) , TokenTree :: Punct ({ let mut punct = Punct :: new ('!' , Spacing :: Alone) ; punct . set_span (self . span) ; punct }) , TokenTree :: Group ({ let mut group = Group :: new (Delimiter :: Brace , { TokenStream :: from_iter (vec ! [TokenTree :: Literal ({ let mut string = Literal :: string (& self . msg) ; string . set_span (self . span) ; string })]) }) ; group . set_span (self . span) ; group }) ,]) } }
};
}
