macro_rules! EmptyLabelManySpans {
    () => {
        pub (crate) struct EmptyLabelManySpans (pub Vec < Span >) ;
    };
}

EmptyLabelManySpans!()