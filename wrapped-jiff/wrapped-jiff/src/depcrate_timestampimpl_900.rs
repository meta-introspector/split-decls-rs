// Generated macro for impl_900 (impl)
macro_rules! Depcrate_timestampimpl_900 {
() => {
// Module: crate::timestamp
// Provides: {"impl_900"}
// Dependencies: {}
impl Iterator for TimestampSeries { type Item = Timestamp ; # [inline] fn next (& mut self) -> Option < Timestamp > { let duration = self . duration ? ; let this = self . ts ; self . ts = self . ts . checked_add_duration (duration) . ok () ? ; Some (this) } }
};
}
