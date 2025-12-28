macro_rules! deps {
    () => {
        TokenStream!();
        Delimiter!();
        Span!();
        Group!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl Group { pub (crate) fn new (delimiter : Delimiter , stream : TokenStream) -> Self { Group { delimiter , stream , span : Span :: call_site () , } } pub (crate) fn delimiter (& self) -> Delimiter { self . delimiter } pub (crate) fn stream (& self) -> TokenStream { self . stream . clone () } pub (crate) fn span (& self) -> Span { self . span } pub (crate) fn span_open (& self) -> Span { self . span . first_byte () } pub (crate) fn span_close (& self) -> Span { self . span . last_byte () } pub (crate) fn set_span (& mut self , span : Span) { self . span = span ; } }
    };
}

impl_115!()