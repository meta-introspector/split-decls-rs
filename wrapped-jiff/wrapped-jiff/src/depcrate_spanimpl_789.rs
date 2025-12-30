// Generated macro for impl_789 (impl)
macro_rules! Depcrate_spanimpl_789 {
() => {
// Module: crate::span
// Provides: {"impl_789"}
// Dependencies: {}
impl < 'a > SpanArithmetic < 'a > { # [doc = " This is a convenience function for setting the relative option on"] # [doc = " this configuration to [`SpanRelativeTo::days_are_24_hours`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " When doing arithmetic on spans involving days, either a relative"] # [doc = " datetime must be provided, or a special assertion opting into 24-hour"] # [doc = " days is required. Otherwise, you get an error."] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::{SpanArithmetic, ToSpan};"] # [doc = ""] # [doc = " let span1 = 2.days().hours(12);"] # [doc = " let span2 = 12.hours();"] # [doc = " // No relative date provided, which results in an error."] # [doc = " assert_eq!("] # [doc = "     span1.checked_add(span2).unwrap_err().to_string(),"] # [doc = "     \"using unit 'day' in a span or configuration requires that \\"] # [doc = "      either a relative reference time be given or \\"] # [doc = "      `SpanRelativeTo::days_are_24_hours()` is used to indicate \\"] # [doc = "      invariant 24-hour days, but neither were provided\","] # [doc = " );"] # [doc = " let sum = span1.checked_add("] # [doc = "     SpanArithmetic::from(span2).days_are_24_hours(),"] # [doc = " )?;"] # [doc = " assert_eq!(sum, 3.days().fieldwise());"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [inline] pub fn days_are_24_hours (self) -> SpanArithmetic < 'a > { self . relative (SpanRelativeTo :: days_are_24_hours ()) } }
};
}
