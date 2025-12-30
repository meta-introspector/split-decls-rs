// Generated macro for impl_236 (impl)
macro_rules! Depcrate_datetimeimpl_236 {
() => {
// Module: crate::datetime
// Provides: {"impl_236"}
// Dependencies: {}
# [cfg (feature = "defmt")] impl < Tz : TimeZone > defmt :: Format for DateTime < Tz > where Tz :: Offset : defmt :: Format , { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "{}{}" , self . overflowing_naive_local () , self . offset) ; } }
};
}
