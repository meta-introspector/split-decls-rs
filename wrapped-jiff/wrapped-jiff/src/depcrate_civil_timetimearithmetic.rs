// Generated macro for TimeArithmetic (struct)
macro_rules! Depcrate_civil_timeTimeArithmetic {
() => {
// Module: crate::civil::time
// Provides: {"TimeArithmetic"}
// Dependencies: {}
# [doc = " Options for [`Time::checked_add`] and [`Time::checked_sub`]."] # [doc = ""] # [doc = " This type provides a way to ergonomically add one of a few different"] # [doc = " duration types to a [`Time`]."] # [doc = ""] # [doc = " The main way to construct values of this type is with its `From` trait"] # [doc = " implementations:"] # [doc = ""] # [doc = " * `From<Span> for TimeArithmetic` adds (or subtracts) the given span to the"] # [doc = " receiver time."] # [doc = " * `From<SignedDuration> for TimeArithmetic` adds (or subtracts)"] # [doc = " the given signed duration to the receiver time."] # [doc = " * `From<std::time::Duration> for TimeArithmetic` adds (or subtracts)"] # [doc = " the given unsigned duration to the receiver time."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " use jiff::{civil::time, SignedDuration, ToSpan};"] # [doc = ""] # [doc = " let t = time(0, 0, 0, 0);"] # [doc = " assert_eq!(t.checked_add(2.hours())?, time(2, 0, 0, 0));"] # [doc = " assert_eq!(t.checked_add(SignedDuration::from_hours(2))?, time(2, 0, 0, 0));"] # [doc = " assert_eq!(t.checked_add(Duration::from_secs(2 * 60 * 60))?, time(2, 0, 0, 0));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct TimeArithmetic { duration : Duration , }
};
}
