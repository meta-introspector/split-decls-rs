// Generated macro for impl_664 (impl)
macro_rules! Depcrate_interpret_stackimpl_664 {
() => {
// Module: crate::interpret::stack
// Provides: {"impl_664"}
// Dependencies: {}
impl SpanGuard { # [doc = " By default a `SpanGuard` does nothing."] fn new () -> Self { Self (tracing :: Span :: none () , std :: marker :: PhantomData) } # [doc = " If a span is entered, we exit the previous span (if any, normally none) and enter the"] # [doc = " new span. This is mainly so we don't have to use `Option` for the `tracing_span` field of"] # [doc = " `Frame` by creating a dummy span to being with and then entering it once the frame has"] # [doc = " been pushed."] fn enter (& mut self , span : tracing :: Span) { * self = Self (span , std :: marker :: PhantomData) ; self . 0 . with_subscriber (| (id , dispatch) | { dispatch . enter (id) ; }) ; } }
};
}
