// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_tz_timezoneimpl_1072 {
() => {
// Module: crate::tz::timezone
// Provides: {"impl_1072"}
// Dependencies: {}
impl < 't > Iterator for TimeZonePrecedingTransitions < 't > { type Item = TimeZoneTransition < 't > ; fn next (& mut self) -> Option < TimeZoneTransition < 't > > { let trans = self . tz . previous_transition (self . cur) ? ; self . cur = trans . timestamp () ; Some (trans) } }
};
}
