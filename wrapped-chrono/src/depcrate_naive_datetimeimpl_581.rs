// Generated macro for impl_581 (impl)
macro_rules! Depcrate_naive_datetimeimpl_581 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_581"}
// Dependencies: {}
# [cfg (feature = "defmt")] impl defmt :: Format for NaiveDateTime { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "{}T{}" , self . date , self . time) ; } }
};
}
