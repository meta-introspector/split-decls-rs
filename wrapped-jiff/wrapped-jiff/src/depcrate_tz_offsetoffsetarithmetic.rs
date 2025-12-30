// Generated macro for OffsetArithmetic (struct)
macro_rules! Depcrate_tz_offsetOffsetArithmetic {
() => {
// Module: crate::tz::offset
// Provides: {"OffsetArithmetic"}
// Dependencies: {}
# [doc = " Options for [`Offset::checked_add`] and [`Offset::checked_sub`]."] # [doc = ""] # [doc = " This type provides a way to ergonomically add one of a few different"] # [doc = " duration types to a [`Offset`]."] # [doc = ""] # [doc = " The main way to construct values of this type is with its `From` trait"] # [doc = " implementations:"] # [doc = ""] # [doc = " * `From<Span> for OffsetArithmetic` adds (or subtracts) the given span to"] # [doc = " the receiver offset."] # [doc = " * `From<SignedDuration> for OffsetArithmetic` adds (or subtracts)"] # [doc = " the given signed duration to the receiver offset."] # [doc = " * `From<std::time::Duration> for OffsetArithmetic` adds (or subtracts)"] # [doc = " the given unsigned duration to the receiver offset."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " use jiff::{tz::offset, SignedDuration, ToSpan};"] # [doc = ""] # [doc = " let off = offset(-10);"] # [doc = " assert_eq!(off.checked_add(11.hours())?, offset(1));"] # [doc = " assert_eq!(off.checked_add(SignedDuration::from_hours(11))?, offset(1));"] # [doc = " assert_eq!(off.checked_add(Duration::from_secs(11 * 60 * 60))?, offset(1));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct OffsetArithmetic { duration : Duration , }
};
}
