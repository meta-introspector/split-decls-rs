// Generated macro for TimeZone (struct)
macro_rules! Depcrate_offset_local_tz_info_timezoneTimeZone {
() => {
// Module: crate::offset::local::tz_info::timezone
// Provides: {"TimeZone"}
// Dependencies: {}
# [doc = " Time zone"] # [derive (Debug , Clone , Eq , PartialEq)] pub (crate) struct TimeZone { # [doc = " List of transitions"] transitions : Vec < Transition > , # [doc = " List of local time types (cannot be empty)"] local_time_types : Vec < LocalTimeType > , # [doc = " List of leap seconds"] leap_seconds : Vec < LeapSecond > , # [doc = " Extra transition rule applicable after the last transition"] extra_rule : Option < TransitionRule > , }
};
}
