// Generated macro for MaybeSpan (trait)
macro_rules! Depcrate_helpersMaybeSpan {
() => {
// Module: crate::helpers
// Provides: {"MaybeSpan"}
// Dependencies: {}
# [doc = " A [`Span`], [`(Span, Span]`], or [`None`]."] pub (crate) trait MaybeSpan { # [doc = " Obtain the span as a start-end pair, falling back to [`Span::call_site()`] if necessary."] fn into_pair (self) -> (Span , Span) ; }
};
}
