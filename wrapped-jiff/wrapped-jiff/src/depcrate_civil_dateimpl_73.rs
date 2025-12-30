// Generated macro for impl_73 (impl)
macro_rules! Depcrate_civil_dateimpl_73 {
() => {
// Module: crate::civil::date
// Provides: {"impl_73"}
// Dependencies: {}
impl From < (Unit , Zoned) > for DateDifference { # [inline] fn from ((largest , zdt) : (Unit , Zoned)) -> DateDifference { DateDifference :: from ((largest , Date :: from (zdt))) } }
};
}
