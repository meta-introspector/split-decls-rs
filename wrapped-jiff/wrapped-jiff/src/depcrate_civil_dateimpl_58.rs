// Generated macro for impl_58 (impl)
macro_rules! Depcrate_civil_dateimpl_58 {
() => {
// Module: crate::civil::date
// Provides: {"impl_58"}
// Dependencies: {}
impl DateArithmetic { # [inline] fn checked_add (self , date : Date) -> Result < Date , Error > { match self . duration . to_signed () ? { SDuration :: Span (span) => date . checked_add_span (span) , SDuration :: Absolute (sdur) => date . checked_add_duration (sdur) , } } # [inline] fn checked_neg (self) -> Result < DateArithmetic , Error > { let duration = self . duration . checked_neg () ? ; Ok (DateArithmetic { duration }) } # [inline] fn is_negative (& self) -> bool { self . duration . is_negative () } }
};
}
