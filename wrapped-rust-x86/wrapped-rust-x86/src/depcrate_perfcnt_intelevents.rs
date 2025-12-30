// Generated macro for events (function)
macro_rules! Depcrate_perfcnt_intelevents {
() => {
// Module: crate::perfcnt::intel
// Provides: {"events"}
// Dependencies: {}
# [doc = " Return all core performance events for the running micro-architecture."] pub fn events () -> Option < & 'static phf :: Map < & 'static str , EventDescription < 'static > > > { get_events ! ("{}-{}-{:X}{:X}") }
};
}
