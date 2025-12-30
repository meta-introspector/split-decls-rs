// Generated macro for impl_196 (impl)
macro_rules! Depcrate_civil_timeimpl_196 {
() => {
// Module: crate::civil::time
// Provides: {"impl_196"}
// Dependencies: {}
impl Iterator for TimeSeries { type Item = Time ; # [inline] fn next (& mut self) -> Option < Time > { let span = self . period . checked_mul (self . step) . ok () ? ; self . step = self . step . checked_add (1) ? ; let time = self . start . checked_add (span) . ok () ? ; Some (time) } }
};
}
