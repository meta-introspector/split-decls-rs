// Generated macro for impl_150 (impl)
macro_rules! Depcrate_fallbackimpl_150 {
() => {
// Module: crate::fallback
// Provides: {"impl_150"}
// Dependencies: {}
impl Group { pub (crate) fn new (delimiter : Delimiter , stream : TokenStream) -> Self { Group { delimiter , stream , span : Span :: call_site () , } } pub (crate) fn delimiter (& self) -> Delimiter { self . delimiter } pub (crate) fn stream (& self) -> TokenStream { self . stream . clone () } pub (crate) fn span (& self) -> Span { self . span } pub (crate) fn span_open (& self) -> Span { self . span . first_byte () } pub (crate) fn span_close (& self) -> Span { self . span . last_byte () } pub (crate) fn set_span (& mut self , span : Span) { self . span = span ; } }
};
}
