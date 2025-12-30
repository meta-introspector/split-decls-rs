// Generated macro for impl_133 (impl)
macro_rules! Depcrate_errimpl_133 {
() => {
// Module: crate::err
// Provides: {"impl_133"}
// Dependencies: {}
impl ParseError { # [doc = " Returns a span of this error, if available. **Note**: the returned span"] # [doc = " might change in future versions of this library. See [the documentation"] # [doc = " of this type][ParseError] for more information."] pub fn span (& self) -> Option < Range < usize > > { self . span . clone () } # [doc = " Adds `offset` to the start and endpoint of the inner span."] pub (crate) fn offset_span (self , offset : usize) -> Self { Self { span : self . span . map (| span | span . start + offset .. span . end + offset) , .. self } } }
};
}
