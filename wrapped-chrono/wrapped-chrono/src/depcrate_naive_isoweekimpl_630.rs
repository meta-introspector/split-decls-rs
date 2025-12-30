// Generated macro for impl_630 (impl)
macro_rules! Depcrate_naive_isoweekimpl_630 {
() => {
// Module: crate::naive::isoweek
// Provides: {"impl_630"}
// Dependencies: {}
# [cfg (feature = "defmt")] impl defmt :: Format for IsoWeek { fn format (& self , fmt : defmt :: Formatter) { let year = self . year () ; let week = self . week () ; if (0 ..= 9999) . contains (& year) { defmt :: write ! (fmt , "{:04}-W{:02}" , year , week) } else { let sign = ['+' , '-'] [(year < 0) as usize] ; defmt :: write ! (fmt , "{}{:05}-W{:02}" , sign , year . abs () , week) } } }
};
}
