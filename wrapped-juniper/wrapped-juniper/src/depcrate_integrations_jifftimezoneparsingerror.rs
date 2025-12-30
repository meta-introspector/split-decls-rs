// Generated macro for TimeZoneParsingError (enum)
macro_rules! Depcrate_integrations_jiffTimeZoneParsingError {
() => {
// Module: crate::integrations::jiff
// Provides: {"TimeZoneParsingError"}
// Dependencies: {}
# [doc = " Error parsing a [`TimeZone`] value."] # [derive (Clone , Debug , Display , Error)] pub enum TimeZoneParsingError { # [doc = " Identifier cannot not be parsed by the [`jiff::tz::TimeZone::get()`] method."] InvalidTimeZone (jiff :: Error) , # [doc = " GraphQL scalar [`TimeZone`] requires `tz::TimeZone` with IANA name."] # [display ("missing IANA name")] MissingIanaName (# [debug (ignore)] # [error (not (source))] jiff :: tz :: TimeZone ,) , }
};
}
