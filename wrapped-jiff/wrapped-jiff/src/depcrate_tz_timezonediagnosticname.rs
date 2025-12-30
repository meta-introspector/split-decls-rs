// Generated macro for DiagnosticName (struct)
macro_rules! Depcrate_tz_timezoneDiagnosticName {
() => {
// Module: crate::tz::timezone
// Provides: {"DiagnosticName"}
// Dependencies: {}
# [doc = " A helper type for converting a `TimeZone` to a succinct human readable"] # [doc = " description."] # [doc = ""] # [doc = " This is principally used in error messages in various places."] # [doc = ""] # [doc = " A previous iteration of this was just an `as_str() -> &str` method on"] # [doc = " `TimeZone`, but that's difficult to do without relying on dynamic memory"] # [doc = " allocation (or chunky arrays)."] pub (crate) struct DiagnosticName < 'a > (& 'a TimeZone) ;
};
}
