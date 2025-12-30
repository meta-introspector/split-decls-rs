// Generated macro for impl_1075 (impl)
macro_rules! Depcrate_tz_timezoneimpl_1075 {
() => {
// Module: crate::tz::timezone
// Provides: {"impl_1075"}
// Dependencies: {}
impl < 't > Iterator for TimeZoneFollowingTransitions < 't > { type Item = TimeZoneTransition < 't > ; fn next (& mut self) -> Option < TimeZoneTransition < 't > > { let trans = self . tz . next_transition (self . cur) ? ; self . cur = trans . timestamp () ; Some (trans) } }
};
}
