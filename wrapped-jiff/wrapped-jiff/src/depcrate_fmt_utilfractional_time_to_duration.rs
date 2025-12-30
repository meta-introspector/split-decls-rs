// Generated macro for fractional_time_to_duration (function)
macro_rules! Depcrate_fmt_utilfractional_time_to_duration {
() => {
// Module: crate::fmt::util
// Provides: {"fractional_time_to_duration"}
// Dependencies: {}
# [doc = " Like `fractional_time_to_span`, but just converts the fraction of the given"] # [doc = " unit to a signed duration."] # [doc = ""] # [doc = " Since a signed duration doesn't keep track of individual units, there is"] # [doc = " no loss of fidelity between it and ISO 8601 durations like there is for"] # [doc = " `Span`."] # [doc = ""] # [doc = " Note that `fraction` can be a fractional hour, minute, second, millisecond"] # [doc = " or microsecond (even though its type suggests it's only a fraction of a"] # [doc = " second). When milliseconds or microseconds, the given fraction has any"] # [doc = " sub-nanosecond precision truncated."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This returns an error if `unit` is not `Hour`, `Minute`, `Second`,"] # [doc = " `Millisecond` or `Microsecond`."] # [inline (never)] fn fractional_time_to_duration (unit : Unit , value : i64 , fraction : i32 ,) -> Result < SignedDuration , Error > { let sdur = duration_unit_value (unit , value) ? ; let fraction_dur = fractional_duration (unit , fraction) ? ; sdur . checked_add (fraction_dur) . ok_or_else (| | { err ! ("accumulated `SignedDuration` of `{sdur:?}` overflowed \
             when adding `{fraction_dur:?}` (from fractional {unit} units)" , unit = unit . singular () ,) }) }
};
}
