// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl Error { fn new (span : Span , message : String) -> Self { Error { span , message } } fn into_compile_error (self) -> TokenStream2 { let mut message = Literal :: string (& self . message) ; message . set_span (self . span) ; quote_spanned ! (self . span => compile_error ! { # message }) } }
};
}
