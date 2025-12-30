// Generated macro for impl_1411 (impl)
macro_rules! Depcrate_zonedimpl_1411 {
() => {
// Module: crate::zoned
// Provides: {"impl_1411"}
// Dependencies: {}
impl Iterator for ZonedSeries { type Item = Zoned ; # [inline] fn next (& mut self) -> Option < Zoned > { loop { let span = self . period . checked_mul (self . step) . ok () ? ; self . step = self . step . checked_add (1) ? ; let zdt = self . start . checked_add (span) . ok () ? ; if self . prev . map_or (true , | prev | { if self . period . is_positive () { prev < zdt . timestamp () } else if self . period . is_negative () { prev > zdt . timestamp () } else { assert ! (self . period . is_zero ()) ; true } }) { self . prev = Some (zdt . timestamp ()) ; return Some (zdt) ; } } } }
};
}
