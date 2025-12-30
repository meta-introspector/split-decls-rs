// Generated macro for walk_span_to_context (function)
macro_rules! Depcrate_sourcewalk_span_to_context {
() => {
// Module: crate::source
// Provides: {"walk_span_to_context"}
// Dependencies: {}
# [doc = " Walks the span up to the target context, thereby returning the macro call site if the span is"] # [doc = " inside a macro expansion, or the original span if it is not."] # [doc = ""] # [doc = " Note this will return `None` in the case of the span being in a macro expansion, but the target"] # [doc = " context is from expanding a macro argument."] # [doc = ""] # [doc = " Given the following"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " macro_rules! m { ($e:expr) => { f($e) }; }"] # [doc = " g(m!(0))"] # [doc = " ```"] # [doc = ""] # [doc = " If called with a span of the call to `f` and a context of the call to `g` this will return a"] # [doc = " span containing `m!(0)`. However, if called with a span of the literal `0` this will give a span"] # [doc = " containing `0` as the context is the same as the outer context."] # [doc = ""] # [doc = " This will traverse through multiple macro calls. Given the following:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " macro_rules! m { ($e:expr) => { n!($e, 0) }; }"] # [doc = " macro_rules! n { ($e:expr, $f:expr) => { f($e, $f) }; }"] # [doc = " g(m!(0))"] # [doc = " ```"] # [doc = ""] # [doc = " If called with a span of the call to `f` and a context of the call to `g` this will return a"] # [doc = " span containing `m!(0)`."] pub fn walk_span_to_context (span : Span , outer : SyntaxContext) -> Option < Span > { let outer_span = hygiene :: walk_chain (span , outer) ; (outer_span . ctxt () == outer) . then_some (outer_span) }
};
}
