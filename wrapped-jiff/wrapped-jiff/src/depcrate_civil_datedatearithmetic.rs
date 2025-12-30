// Generated macro for DateArithmetic (struct)
macro_rules! Depcrate_civil_dateDateArithmetic {
() => {
// Module: crate::civil::date
// Provides: {"DateArithmetic"}
// Dependencies: {}
# [doc = " Options for [`Date::checked_add`] and [`Date::checked_sub`]."] # [doc = ""] # [doc = " This type provides a way to ergonomically add one of a few different"] # [doc = " duration types to a [`Date`]."] # [doc = ""] # [doc = " The main way to construct values of this type is with its `From` trait"] # [doc = " implementations:"] # [doc = ""] # [doc = " * `From<Span> for DateArithmetic` adds (or subtracts) the given span to the"] # [doc = " receiver date."] # [doc = " * `From<SignedDuration> for DateArithmetic` adds (or subtracts)"] # [doc = " the given signed duration to the receiver date."] # [doc = " * `From<std::time::Duration> for DateArithmetic` adds (or subtracts)"] # [doc = " the given unsigned duration to the receiver date."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " use jiff::{civil::date, SignedDuration, ToSpan};"] # [doc = ""] # [doc = " let d = date(2024, 2, 29);"] # [doc = " assert_eq!(d.checked_add(1.year())?, date(2025, 2, 28));"] # [doc = " assert_eq!(d.checked_add(SignedDuration::from_hours(24))?, date(2024, 3, 1));"] # [doc = " assert_eq!(d.checked_add(Duration::from_secs(24 * 60 * 60))?, date(2024, 3, 1));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct DateArithmetic { duration : Duration , }
};
}
