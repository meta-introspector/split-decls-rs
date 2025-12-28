macro_rules! SpanGuard {
    () => {
        struct SpanGuard (tracing :: Span , std :: marker :: PhantomData < * const u8 >) ;
    };
}

SpanGuard!();