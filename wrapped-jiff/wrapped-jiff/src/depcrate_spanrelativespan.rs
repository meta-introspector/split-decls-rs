// Generated macro for RelativeSpan (struct)
macro_rules! Depcrate_spanRelativeSpan {
() => {
// Module: crate::span
// Provides: {"RelativeSpan"}
// Dependencies: {}
# [doc = " A balanced span between a range of civil or zoned datetimes."] # [doc = ""] # [doc = " The span is always balanced up to a certain unit as given to"] # [doc = " `RelativeSpanKind::into_relative_span`."] # [derive (Clone , Debug)] struct RelativeSpan < 'a > { span : Span , kind : RelativeSpanKind < 'a > , }
};
}
