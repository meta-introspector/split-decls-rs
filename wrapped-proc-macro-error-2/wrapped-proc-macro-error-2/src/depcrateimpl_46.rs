// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl SpanRange { # [doc = " Create a range with the `first` and `last` spans being the same."] pub fn single_span (span : Span) -> Self { SpanRange { first : span , last : span , } } # [doc = " Create a `SpanRange` resolving at call site."] pub fn call_site () -> Self { SpanRange :: single_span (Span :: call_site ()) } # [doc = " Construct span range from a `TokenStream`. This method always preserves all the"] # [doc = " range."] # [doc = ""] # [doc = " ### Note"] # [doc = ""] # [doc = " If the stream is empty, the result is `SpanRange::call_site()`. If the stream"] # [doc = " consists of only one `TokenTree`, the result is `SpanRange::single_span(tt.span())`"] # [doc = " that doesn't lose anything."] pub fn from_tokens (ts : & dyn ToTokens) -> Self { let mut spans = ts . to_token_stream () . into_iter () . map (| tt | tt . span ()) ; let first = spans . next () . unwrap_or_else (Span :: call_site) ; let last = spans . last () . unwrap_or (first) ; SpanRange { first , last } } # [doc = " Join two span ranges. The resulting range will start at `self.first` and end at"] # [doc = " `other.last`."] pub fn join_range (self , other : SpanRange) -> Self { SpanRange { first : self . first , last : other . last , } } # [doc = " Collapse the range into single span, preserving as much information as possible."] # [must_use] pub fn collapse (self) -> Span { self . first . join (self . last) . unwrap_or (self . first) } }
};
}
