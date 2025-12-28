macro_rules! deps {
    () => {
        SpanMap!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl syntax_bridge :: SpanMapper < Span > for SpanMap { fn span_for (& self , range : TextRange) -> Span { self . span_for_range (range) } }
    };
}

impl_181!()