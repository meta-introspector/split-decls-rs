// Generated macro for impl_903 (impl)
macro_rules! Depcrate_timestampimpl_903 {
() => {
// Module: crate::timestamp
// Provides: {"impl_903"}
// Dependencies: {}
impl TimestampArithmetic { # [inline] fn checked_add (self , ts : Timestamp) -> Result < Timestamp , Error > { match self . duration . to_signed () ? { SDuration :: Span (span) => ts . checked_add_span (span) , SDuration :: Absolute (sdur) => ts . checked_add_duration (sdur) , } } # [inline] fn saturating_add (self , ts : Timestamp) -> Result < Timestamp , Error > { let Ok (signed) = self . duration . to_signed () else { return Ok (Timestamp :: MAX) ; } ; let result = match signed { SDuration :: Span (span) => { if let Some (err) = span . smallest_non_time_non_zero_unit_error () { return Err (err) ; } ts . checked_add_span (span) } SDuration :: Absolute (sdur) => ts . checked_add_duration (sdur) , } ; Ok (result . unwrap_or_else (| _ | { if self . is_negative () { Timestamp :: MIN } else { Timestamp :: MAX } })) } # [inline] fn checked_neg (self) -> Result < TimestampArithmetic , Error > { let duration = self . duration . checked_neg () ? ; Ok (TimestampArithmetic { duration }) } # [inline] fn is_negative (& self) -> bool { self . duration . is_negative () } }
};
}
