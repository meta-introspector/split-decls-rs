// Generated macro for impl_77 (impl)
macro_rules! Depcrate_dateimpl_77 {
() => {
// Module: crate::date
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "defmt")] impl < Tz : TimeZone > defmt :: Format for Date < Tz > where Tz :: Offset : defmt :: Format , { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "{}{}" , self . naive_local () , self . offset) ; } }
};
}
