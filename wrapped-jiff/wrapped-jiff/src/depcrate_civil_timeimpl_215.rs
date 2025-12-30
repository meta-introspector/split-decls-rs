// Generated macro for impl_215 (impl)
macro_rules! Depcrate_civil_timeimpl_215 {
() => {
// Module: crate::civil::time
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'a > From < (Unit , & 'a Zoned) > for TimeDifference { # [inline] fn from ((largest , zdt) : (Unit , & 'a Zoned)) -> TimeDifference { TimeDifference :: from ((largest , zdt . datetime ())) } }
};
}
