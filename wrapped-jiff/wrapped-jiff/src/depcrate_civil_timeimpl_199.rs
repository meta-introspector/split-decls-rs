// Generated macro for impl_199 (impl)
macro_rules! Depcrate_civil_timeimpl_199 {
() => {
// Module: crate::civil::time
// Provides: {"impl_199"}
// Dependencies: {}
impl TimeArithmetic { # [inline] fn wrapping_add (self , time : Time) -> Time { match self . duration { Duration :: Span (span) => time . wrapping_add_span (span) , Duration :: Signed (sdur) => time . wrapping_add_signed_duration (sdur) , Duration :: Unsigned (udur) => { time . wrapping_add_unsigned_duration (udur) } } } # [inline] fn wrapping_sub (self , time : Time) -> Time { match self . duration { Duration :: Span (span) => time . wrapping_add_span (span . negate ()) , Duration :: Signed (sdur) => { if let Some (sdur) = sdur . checked_neg () { time . wrapping_add_signed_duration (sdur) } else { let udur = UnsignedDuration :: new (i64 :: MIN . unsigned_abs () , sdur . subsec_nanos () . unsigned_abs () ,) ; time . wrapping_add_unsigned_duration (udur) } } Duration :: Unsigned (udur) => { time . wrapping_sub_unsigned_duration (udur) } } } # [inline] fn checked_add (self , time : Time) -> Result < Time , Error > { match self . duration . to_signed () ? { SDuration :: Span (span) => time . checked_add_span (span) , SDuration :: Absolute (sdur) => time . checked_add_duration (sdur) , } } # [inline] fn checked_neg (self) -> Result < TimeArithmetic , Error > { let duration = self . duration . checked_neg () ? ; Ok (TimeArithmetic { duration }) } # [inline] fn is_negative (& self) -> bool { self . duration . is_negative () } }
};
}
