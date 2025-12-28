macro_rules! deps {
    () => {
        SpanMap!();
    };
}

macro_rules! ExpansionSpanMap {
    () => {
        deps!();
        pub type ExpansionSpanMap = span :: SpanMap < SyntaxContext > ;
    };
}

ExpansionSpanMap!();