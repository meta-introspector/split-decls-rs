// Generated macro for Span (struct)
macro_rules! Depcrate_timezone_implSpan {
() => {
// Module: crate::timezone_impl
// Provides: {"Span"}
// Dependencies: {}
# [doc = " Represents the span of time that a given rule is valid for."] # [doc = " Note that I have made the assumption that all ranges are"] # [doc = " left-inclusive and right-exclusive - that is to say,"] # [doc = " if the clocks go forward by 1 hour at 1am, the time 1am"] # [doc = " does not exist in local time (the clock goes from 00:59:59"] # [doc = " to 02:00:00). Likewise, if the clocks go back by one hour"] # [doc = " at 2am, the clock goes from 01:59:59 to 01:00:00. This is"] # [doc = " an arbitrary choice, and I could not find a source to"] # [doc = " confirm whether or not this is correct."] struct Span { begin : Option < i64 > , end : Option < i64 > , }
};
}
