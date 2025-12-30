// Generated macro for SpanCompare (struct)
macro_rules! Depcrate_spanSpanCompare {
() => {
// Module: crate::span
// Provides: {"SpanCompare"}
// Dependencies: {}
# [doc = " Options for [`Span::compare`]."] # [doc = ""] # [doc = " This type provides a way to ergonomically compare two spans with an"] # [doc = " optional relative datetime. Namely, a relative datetime is only needed when"] # [doc = " at least one of the two spans being compared has a non-zero calendar unit"] # [doc = " (years, months, weeks or days). Otherwise, an error will be returned."] # [doc = ""] # [doc = " Callers may use [`SpanCompare::days_are_24_hours`] to opt into 24-hour"] # [doc = " invariant days (and 7-day weeks) without providing a relative datetime."] # [doc = ""] # [doc = " The main way to construct values of this type is with its `From` trait"] # [doc = " implementations:"] # [doc = ""] # [doc = " * `From<Span> for SpanCompare` compares the given span to the receiver"] # [doc = " in [`Span::compare`]."] # [doc = " * `From<(Span, civil::Date)> for SpanCompare` compares the given span"] # [doc = " to the receiver in [`Span::compare`], relative to the given date. There"] # [doc = " are also `From` implementations for `civil::DateTime`, `Zoned` and"] # [doc = " [`SpanRelativeTo`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::ToSpan;"] # [doc = ""] # [doc = " let span1 = 3.hours();"] # [doc = " let span2 = 180.minutes();"] # [doc = " assert_eq!(span1.compare(span2)?, std::cmp::Ordering::Equal);"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct SpanCompare < 'a > { span : Span , relative : Option < SpanRelativeTo < 'a > > , }
};
}
