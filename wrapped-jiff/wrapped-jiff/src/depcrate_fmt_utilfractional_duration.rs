// Generated macro for fractional_duration (function)
macro_rules! Depcrate_fmt_utilfractional_duration {
() => {
// Module: crate::fmt::util
// Provides: {"fractional_duration"}
// Dependencies: {}
# [doc = " Converts the fraction of the given unit to a signed duration."] # [doc = ""] # [doc = " Since a signed duration doesn't keep track of individual units, there is"] # [doc = " no loss of fidelity between it and ISO 8601 durations like there is for"] # [doc = " `Span`. Thus, we can do something far less complicated."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When `fraction` isn't in the range `-999_999_999..=999_999_999`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This returns an error if `unit` is not `Hour`, `Minute`, `Second`,"] # [doc = " `Millisecond` or `Microsecond`."] # [inline (never)] fn fractional_duration (unit : Unit , fraction : i32 ,) -> Result < SignedDuration , Error > { let fraction = i64 :: from (fraction) ; let nanos = match unit { Unit :: Hour => fraction * t :: SECONDS_PER_HOUR . value () , Unit :: Minute => fraction * t :: SECONDS_PER_MINUTE . value () , Unit :: Second => fraction , Unit :: Millisecond => fraction / t :: NANOS_PER_MICRO . value () , Unit :: Microsecond => fraction / t :: NANOS_PER_MILLI . value () , unit => { return Err (err ! ("fractional {unit} units are not allowed" , unit = unit . singular () ,)) } } ; Ok (SignedDuration :: from_nanos (nanos)) }
};
}
