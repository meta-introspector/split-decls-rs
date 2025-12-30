// Generated macro for impl_210 (impl)
macro_rules! Depcrate_civil_timeimpl_210 {
() => {
// Module: crate::civil::time
// Provides: {"impl_210"}
// Dependencies: {}
impl From < Zoned > for TimeDifference { # [inline] fn from (zdt : Zoned) -> TimeDifference { TimeDifference :: from (Time :: from (zdt)) } }
};
}
