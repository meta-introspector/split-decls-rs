// Generated macro for impl_209 (impl)
macro_rules! Depcrate_civil_timeimpl_209 {
() => {
// Module: crate::civil::time
// Provides: {"impl_209"}
// Dependencies: {}
impl From < DateTime > for TimeDifference { # [inline] fn from (dt : DateTime) -> TimeDifference { TimeDifference :: from (Time :: from (dt)) } }
};
}
