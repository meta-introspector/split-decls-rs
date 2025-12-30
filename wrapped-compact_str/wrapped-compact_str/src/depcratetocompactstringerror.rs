// Generated macro for ToCompactStringError (enum)
macro_rules! DepcrateToCompactStringError {
() => {
// Module: crate
// Provides: {"ToCompactStringError"}
// Dependencies: {}
# [doc = " A possible error value if [`ToCompactString::try_to_compact_string()`] failed."] # [derive (Debug , Clone , Copy , PartialEq)] # [non_exhaustive] pub enum ToCompactStringError { # [doc = " Cannot allocate memory to hold CompactString"] Reserve (ReserveError) , # [doc = " [`Display::fmt()`][core::fmt::Display::fmt] returned an error"] Fmt (fmt :: Error) , }
};
}
