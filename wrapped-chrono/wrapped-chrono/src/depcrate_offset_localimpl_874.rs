// Generated macro for impl_874 (impl)
macro_rules! Depcrate_offset_localimpl_874 {
() => {
// Module: crate::offset::local
// Provides: {"impl_874"}
// Dependencies: {}
# [cfg (windows)] impl Transition { fn new (transition_local : NaiveDateTime , offset_before : FixedOffset , offset_after : FixedOffset ,) -> Transition { let transition_utc = transition_local . overflowing_sub_offset (offset_before) ; Transition { transition_utc , offset_before , offset_after } } }
};
}
