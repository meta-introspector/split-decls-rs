// Generated macro for impl_1414 (impl)
macro_rules! Depcrate_zonedimpl_1414 {
() => {
// Module: crate::zoned
// Provides: {"impl_1414"}
// Dependencies: {}
impl ZonedArithmetic { # [inline] fn checked_add (self , zdt : & Zoned) -> Result < Zoned , Error > { match self . duration . to_signed () ? { SDuration :: Span (span) => zdt . checked_add_span (span) , SDuration :: Absolute (sdur) => zdt . checked_add_duration (sdur) , } } # [inline] fn checked_neg (self) -> Result < ZonedArithmetic , Error > { let duration = self . duration . checked_neg () ? ; Ok (ZonedArithmetic { duration }) } # [inline] fn is_negative (& self) -> bool { self . duration . is_negative () } }
};
}
