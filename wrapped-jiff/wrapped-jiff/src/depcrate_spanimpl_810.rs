// Generated macro for impl_810 (impl)
macro_rules! Depcrate_spanimpl_810 {
() => {
// Module: crate::span
// Provides: {"impl_810"}
// Dependencies: {}
impl < 'a > SpanCompare < 'a > { # [doc = " This is a convenience function for setting the relative option on"] # [doc = " this configuration to [`SpanRelativeTo::days_are_24_hours`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " When comparing spans involving days, either a relative datetime must be"] # [doc = " provided, or a special assertion opting into 24-hour days is"] # [doc = " required. Otherwise, you get an error."] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::{SpanCompare, ToSpan, Unit};"] # [doc = ""] # [doc = " let span1 = 2.days().hours(12);"] # [doc = " let span2 = 60.hours();"] # [doc = " // No relative date provided, which results in an error."] # [doc = " assert_eq!("] # [doc = "     span1.compare(span2).unwrap_err().to_string(),"] # [doc = "     \"using unit 'day' in a span or configuration requires that \\"] # [doc = "      either a relative reference time be given or \\"] # [doc = "      `SpanRelativeTo::days_are_24_hours()` is used to indicate \\"] # [doc = "      invariant 24-hour days, but neither were provided\","] # [doc = " );"] # [doc = " let ordering = span1.compare("] # [doc = "     SpanCompare::from(span2).days_are_24_hours(),"] # [doc = " )?;"] # [doc = " assert_eq!(ordering, std::cmp::Ordering::Equal);"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [inline] pub fn days_are_24_hours (self) -> SpanCompare < 'a > { self . relative (SpanRelativeTo :: days_are_24_hours ()) } }
};
}
