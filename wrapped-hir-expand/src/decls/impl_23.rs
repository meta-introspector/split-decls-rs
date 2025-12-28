macro_rules! deps {
    () => {
        ExpandErrorKind!();
        RenderedExpandError!();
        ExpandError!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl ExpandError { pub fn new (span : Span , kind : ExpandErrorKind) -> Self { ExpandError { inner : Arc :: new ((kind , span)) } } pub fn other (span : Span , msg : impl Into < Box < str > >) -> Self { ExpandError { inner : Arc :: new ((ExpandErrorKind :: Other (msg . into ()) , span)) } } pub fn kind (& self) -> & ExpandErrorKind { & self . inner . 0 } pub fn span (& self) -> Span { self . inner . 1 } pub fn render_to_string (& self , db : & dyn ExpandDatabase) -> RenderedExpandError { self . inner . 0 . render_to_string (db) } }
    };
}

impl_23!()