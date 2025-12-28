macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl ParseError { # [doc = " Returns a span of this error, if available. **Note**: the returned span"] # [doc = " might change in future versions of this library. See [the documentation"] # [doc = " of this type][ParseError] for more information."] pub fn span (& self) -> Option < Range < usize > > { self . span . clone () } # [doc = " Adds `offset` to the start and endpoint of the inner span."] pub (crate) fn offset_span (self , offset : usize) -> Self { Self { span : self . span . map (| span | span . start + offset .. span . end + offset) , .. self } } }
    };
}

impl_110!()