// Generated macro for impl_115 (impl)
macro_rules! Depcrate_civil_datetimeimpl_115 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_115"}
// Dependencies: {}
impl Iterator for DateTimeSeries { type Item = DateTime ; # [inline] fn next (& mut self) -> Option < DateTime > { let span = self . period . checked_mul (self . step) . ok () ? ; self . step = self . step . checked_add (1) ? ; let date = self . start . checked_add (span) . ok () ? ; Some (date) } }
};
}
