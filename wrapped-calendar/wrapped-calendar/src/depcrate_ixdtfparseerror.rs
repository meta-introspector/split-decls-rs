// Generated macro for ParseError (enum)
macro_rules! Depcrate_ixdtfParseError {
() => {
// Module: crate::ixdtf
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " An error returned from parsing an RFC 9557 string to an `icu::calendar` type."] # [derive (Debug , displaydoc :: Display)] # [non_exhaustive] pub enum ParseError { # [doc = " Syntax error."] # [displaydoc ("Syntax error in the RFC 9557 string: {0}")] Syntax (Rfc9557Error) , # [doc = " Value is out of range."] # [displaydoc ("Value out of range: {0}")] Range (RangeError) , # [doc = " The RFC 9557 string is missing fields required for parsing into the chosen type."] MissingFields , # [doc = " The RFC 9557 string specifies an unknown calendar."] UnknownCalendar , # [doc = " Expected a different calendar."] # [displaydoc ("Expected calendar {0:?} but found calendar {1:?}")] MismatchedCalendar (CalendarAlgorithm , CalendarAlgorithm) , }
};
}
