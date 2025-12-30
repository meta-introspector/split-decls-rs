// Generated macro for impl_55 (impl)
macro_rules! Depcrate_civil_dateimpl_55 {
() => {
// Module: crate::civil::date
// Provides: {"impl_55"}
// Dependencies: {}
impl Iterator for DateSeries { type Item = Date ; # [inline] fn next (& mut self) -> Option < Date > { let span = self . period . checked_mul (self . step) . ok () ? ; self . step = self . step . checked_add (1) ? ; let date = self . start . checked_add (span) . ok () ? ; Some (date) } }
};
}
