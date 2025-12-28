macro_rules! deps {
    () => {
        ExpansionSpanMap!();
        SpanMapRef!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl SpanMapRef < '_ > { pub fn span_for_range (self , range : TextRange) -> Span { match self { Self :: ExpansionSpanMap (span_map) => span_map . span_at (range . start ()) , Self :: RealSpanMap (span_map) => span_map . span_for_range (range) , } } }
    };
}

impl_184!()