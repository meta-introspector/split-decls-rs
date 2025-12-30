// Generated macro for impl_214 (impl)
macro_rules! Depcrate_civil_timeimpl_214 {
() => {
// Module: crate::civil::time
// Provides: {"impl_214"}
// Dependencies: {}
impl From < (Unit , Zoned) > for TimeDifference { # [inline] fn from ((largest , zdt) : (Unit , Zoned)) -> TimeDifference { TimeDifference :: from ((largest , Time :: from (zdt))) } }
};
}
