// Generated macro for MismatchedCalendarError (struct)
macro_rules! Depcrate_errorMismatchedCalendarError {
() => {
// Module: crate::error
// Provides: {"MismatchedCalendarError"}
// Dependencies: {}
# [doc = " An error from mixing calendar types in a formatter."] # [derive (Display , Debug , Copy , Clone , PartialEq)] # [displaydoc ("DateTimeFormatter for {this_kind} calendar was given a {date_kind:?} calendar")] # [non_exhaustive] pub struct MismatchedCalendarError { # [doc = " The calendar kind of the target object (formatter)."] pub this_kind : AnyCalendarKind , # [doc = " The calendar kind of the input object (date being formatted)."] # [doc = " Can be `None` if the input calendar was not specified."] pub date_kind : Option < AnyCalendarKind > , }
};
}
