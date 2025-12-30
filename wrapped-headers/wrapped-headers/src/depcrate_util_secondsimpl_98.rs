// Generated macro for impl_98 (impl)
macro_rules! Depcrate_util_secondsimpl_98 {
() => {
// Module: crate::util::seconds
// Provides: {"impl_98"}
// Dependencies: {}
impl From < Duration > for Seconds { fn from (dur : Duration) -> Seconds { debug_assert ! (dur . subsec_nanos () == 0) ; Seconds (dur) } }
};
}
