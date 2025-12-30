// Generated macro for TimeZoneRef (struct)
macro_rules! Depcrate_offset_local_tz_info_timezoneTimeZoneRef {
() => {
// Module: crate::offset::local::tz_info::timezone
// Provides: {"TimeZoneRef"}
// Dependencies: {}
# [doc = " Reference to a time zone"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) struct TimeZoneRef < 'a > { # [doc = " List of transitions"] transitions : & 'a [Transition] , # [doc = " List of local time types (cannot be empty)"] local_time_types : & 'a [LocalTimeType] , # [doc = " List of leap seconds"] leap_seconds : & 'a [LeapSecond] , # [doc = " Extra transition rule applicable after the last transition"] extra_rule : & 'a Option < TransitionRule > , }
};
}
