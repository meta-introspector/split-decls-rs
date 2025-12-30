// Generated macro for impl_118 (impl)
macro_rules! Depcrate_civil_datetimeimpl_118 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_118"}
// Dependencies: {}
impl DateTimeArithmetic { # [inline] fn checked_add (self , dt : DateTime) -> Result < DateTime , Error > { match self . duration . to_signed () ? { SDuration :: Span (span) => dt . checked_add_span (span) , SDuration :: Absolute (sdur) => dt . checked_add_duration (sdur) , } } # [inline] fn checked_neg (self) -> Result < DateTimeArithmetic , Error > { let duration = self . duration . checked_neg () ? ; Ok (DateTimeArithmetic { duration }) } # [inline] fn is_negative (& self) -> bool { self . duration . is_negative () } }
};
}
