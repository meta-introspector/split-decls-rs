// Generated macro for impl_292 (impl)
macro_rules! Depcrate_pattern_namesimpl_292 {
() => {
// Module: crate::pattern::names
// Provides: {"impl_292"}
// Dependencies: {}
impl < FSet : DateTimeNamesMarker > fmt :: Debug for RawDateTimeNames < FSet > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawDateTimeNames") . field ("year_names" , & self . year_names) . field ("month_names" , & self . month_names) . field ("weekday_names" , & self . weekday_names) . field ("dayperiod_names" , & self . dayperiod_names) . field ("zone_essentials" , & self . zone_essentials) . field ("locations_root" , & self . locations_root) . field ("locations" , & self . locations) . field ("exemplars_root" , & self . exemplars_root) . field ("exemplars" , & self . exemplars) . field ("mz_generic_long" , & self . mz_generic_long) . field ("mz_generic_short" , & self . mz_generic_short) . field ("mz_standard_long" , & self . mz_standard_long) . field ("mz_specific_long" , & self . mz_specific_long) . field ("mz_specific_short" , & self . mz_specific_short) . field ("mz_periods" , & self . mz_periods) . field ("decimal_formatter" , & self . decimal_formatter) . finish () } }
};
}
