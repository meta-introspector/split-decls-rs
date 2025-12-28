macro_rules! deps {
    () => {
        Span!();
        FrontmatterError!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl FrontmatterError { pub fn new (message : impl Into < String > , span : Span) -> Self { Self { message : message . into () , primary_span : span , visible_spans : Vec :: new () } } pub fn push_visible_span (mut self , span : Span) -> Self { self . visible_spans . push (span) ; self } pub fn message (& self) -> & str { self . message . as_str () } pub fn primary_span (& self) -> Span { self . primary_span . clone () } pub fn visible_spans (& self) -> & [Span] { & self . visible_spans } }
    };
}

impl_15!()