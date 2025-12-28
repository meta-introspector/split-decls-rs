macro_rules! deps {
    () => {
        SpanMapRef!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl syntax_bridge :: SpanMapper < Span > for SpanMapRef < '_ > { fn span_for (& self , range : TextRange) -> Span { self . span_for_range (range) } }
    };
}

impl_182!()