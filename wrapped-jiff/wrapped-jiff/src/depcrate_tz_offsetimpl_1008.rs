// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_tz_offsetimpl_1008 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1008"}
// Dependencies: {}
impl OffsetArithmetic { # [inline] fn checked_add (self , offset : Offset) -> Result < Offset , Error > { match self . duration . to_signed () ? { SDuration :: Span (span) => offset . checked_add_span (span) , SDuration :: Absolute (sdur) => offset . checked_add_duration (sdur) , } } # [inline] fn checked_neg (self) -> Result < OffsetArithmetic , Error > { let duration = self . duration . checked_neg () ? ; Ok (OffsetArithmetic { duration }) } # [inline] fn is_negative (& self) -> bool { self . duration . is_negative () } }
};
}
