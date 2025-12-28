macro_rules! CastUnknownPointerSub {
    () => {
        pub (crate) enum CastUnknownPointerSub { To (Span) , From (Span) , }
    };
}

CastUnknownPointerSub!();