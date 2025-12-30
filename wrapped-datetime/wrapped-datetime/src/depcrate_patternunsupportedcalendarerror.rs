// Generated macro for UnsupportedCalendarError (struct)
macro_rules! Depcrate_patternUnsupportedCalendarError {
() => {
// Module: crate::pattern
// Provides: {"UnsupportedCalendarError"}
// Dependencies: {}
# [doc = " Error returned from constructors that map from AnyCalendar to a formatter."] # [derive (Debug , Clone , Copy , PartialEq , displaydoc :: Display)] # [displaydoc ("The calendar {kind:?} is not supported in DateTimeFormatter")] # [non_exhaustive] pub struct UnsupportedCalendarError { # [doc = " The calendar kind that is not supported."] pub kind : AnyCalendarKind , }
};
}
