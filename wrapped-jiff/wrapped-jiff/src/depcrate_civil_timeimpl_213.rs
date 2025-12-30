// Generated macro for impl_213 (impl)
macro_rules! Depcrate_civil_timeimpl_213 {
() => {
// Module: crate::civil::time
// Provides: {"impl_213"}
// Dependencies: {}
impl From < (Unit , DateTime) > for TimeDifference { # [inline] fn from ((largest , dt) : (Unit , DateTime)) -> TimeDifference { TimeDifference :: from ((largest , Time :: from (dt))) } }
};
}
