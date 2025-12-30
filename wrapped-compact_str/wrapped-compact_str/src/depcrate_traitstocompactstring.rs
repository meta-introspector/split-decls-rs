// Generated macro for ToCompactString (trait)
macro_rules! Depcrate_traitsToCompactString {
() => {
// Module: crate::traits
// Provides: {"ToCompactString"}
// Dependencies: {}
# [doc = " A trait for converting a value to a `CompactString`."] # [doc = ""] # [doc = " This trait is automatically implemented for any type which implements the"] # [doc = " [`fmt::Display`] trait. As such, [`ToCompactString`] shouldn't be implemented directly:"] # [doc = " [`fmt::Display`] should be implemented instead, and you get the [`ToCompactString`]"] # [doc = " implementation for free."] pub trait ToCompactString { # [doc = " Converts the given value to a [`CompactString`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the system runs out of memory and it cannot hold the whole string,"] # [doc = " or if [`Display::fmt()`][core::fmt::Display::fmt] returns an error."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use compact_str::ToCompactString;"] # [doc = " # use compact_str::CompactString;"] # [doc = ""] # [doc = " let i = 5;"] # [doc = " let five = CompactString::new(\"5\");"] # [doc = ""] # [doc = " assert_eq!(i.to_compact_string(), five);"] # [doc = " ```"] # [inline] # [track_caller] fn to_compact_string (& self) -> CompactString { self . try_to_compact_string () . unwrap_with_msg () } # [doc = " Fallible version of [`ToCompactString::to_compact_string()`]"] # [doc = ""] # [doc = " This method won't panic if the system is out-of-memory, but return a"] # [doc = " [`ReserveError`][crate::ReserveError]."] # [doc = " Otherwise it behaves the same as [`ToCompactString::to_compact_string()`]."] fn try_to_compact_string (& self) -> Result < CompactString , ToCompactStringError > ; }
};
}
