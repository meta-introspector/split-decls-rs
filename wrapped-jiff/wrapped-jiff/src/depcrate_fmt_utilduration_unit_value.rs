// Generated macro for duration_unit_value (function)
macro_rules! Depcrate_fmt_utilduration_unit_value {
() => {
// Module: crate::fmt::util
// Provides: {"duration_unit_value"}
// Dependencies: {}
# [doc = " Returns the given parsed value, interpreted as the given unit, as a"] # [doc = " `SignedDuration`."] # [doc = ""] # [doc = " If the given unit is not supported for signed durations (i.e., calendar"] # [doc = " units), or if converting the given value to a `SignedDuration` for the"] # [doc = " given units overflows, then an error is returned."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn duration_unit_value (unit : Unit , value : i64 ,) -> Result < SignedDuration , Error > { let sdur = match unit { Unit :: Hour => { let seconds = value . checked_mul (t :: SECONDS_PER_HOUR . value ()) . ok_or_else (| | { err ! ("converting {value} hours to seconds overflows i64") }) ? ; SignedDuration :: from_secs (seconds) } Unit :: Minute => { let seconds = value . checked_mul (t :: SECONDS_PER_MINUTE . value ()) . ok_or_else (| | { err ! ("converting {value} minutes to seconds overflows i64") }) ? ; SignedDuration :: from_secs (seconds) } Unit :: Second => SignedDuration :: from_secs (value) , Unit :: Millisecond => SignedDuration :: from_millis (value) , Unit :: Microsecond => SignedDuration :: from_micros (value) , Unit :: Nanosecond => SignedDuration :: from_nanos (value) , unsupported => { return Err (err ! ("parsing {unit} units into a `SignedDuration` is not supported \
                 (perhaps try parsing into a `Span` instead)" , unit = unsupported . singular () ,)) ; } } ; Ok (sdur) }
};
}
