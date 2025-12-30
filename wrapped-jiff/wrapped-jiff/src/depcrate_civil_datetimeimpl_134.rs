// Generated macro for impl_134 (impl)
macro_rules! Depcrate_civil_datetimeimpl_134 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a > From < (Unit , & 'a Zoned) > for DateTimeDifference { # [inline] fn from ((largest , zdt) : (Unit , & 'a Zoned)) -> DateTimeDifference { DateTimeDifference :: from ((largest , zdt . datetime ())) } }
};
}
