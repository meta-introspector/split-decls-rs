macro_rules! deps {
    () => {
        Transition!();
        LocalTimeType!();
        TransitionRule!();
        LeapSecond!();
    };
}

macro_rules! TimeZoneRef {
    () => {
        deps!();
        # [doc = " Reference to a time zone"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) struct TimeZoneRef < 'a > { # [doc = " List of transitions"] transitions : & 'a [Transition] , # [doc = " List of local time types (cannot be empty)"] local_time_types : & 'a [LocalTimeType] , # [doc = " List of leap seconds"] leap_seconds : & 'a [LeapSecond] , # [doc = " Extra transition rule applicable after the last transition"] extra_rule : & 'a Option < TransitionRule > , }
    };
}

TimeZoneRef!();