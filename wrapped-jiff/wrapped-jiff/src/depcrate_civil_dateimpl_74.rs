// Generated macro for impl_74 (impl)
macro_rules! Depcrate_civil_dateimpl_74 {
() => {
// Module: crate::civil::date
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a > From < (Unit , & 'a Zoned) > for DateDifference { # [inline] fn from ((largest , zdt) : (Unit , & 'a Zoned)) -> DateDifference { DateDifference :: from ((largest , zdt . datetime ())) } }
};
}
