// Generated macro for impl_294 (impl)
macro_rules! Depcrate_pattern_namesimpl_294 {
() => {
// Module: crate::pattern::names
// Provides: {"impl_294"}
// Dependencies: {}
impl < FSet : DateTimeNamesMarker > RawDateTimeNames < FSet > { pub (crate) fn cast_into_fset < FSet2 : DateTimeNamesFrom < FSet > > (self) -> RawDateTimeNames < FSet2 > { RawDateTimeNames { year_names : FSet2 :: map_year_names (self . year_names) , month_names : FSet2 :: map_month_names (self . month_names) , weekday_names : FSet2 :: map_weekday_names (self . weekday_names) , dayperiod_names : FSet2 :: map_day_period_names (self . dayperiod_names) , zone_essentials : FSet2 :: map_zone_essentials (self . zone_essentials) , locations_root : FSet2 :: map_zone_locations_root (self . locations_root) , locations : FSet2 :: map_zone_locations (self . locations) , exemplars_root : FSet2 :: map_zone_exemplars_root (self . exemplars_root) , exemplars : FSet2 :: map_zone_exemplars (self . exemplars) , mz_generic_long : FSet2 :: map_zone_generic_long (self . mz_generic_long) , mz_generic_short : FSet2 :: map_zone_generic_short (self . mz_generic_short) , mz_standard_long : FSet2 :: map_zone_standard_long (self . mz_standard_long) , mz_specific_long : FSet2 :: map_zone_specific_long (self . mz_specific_long) , mz_specific_short : FSet2 :: map_zone_specific_short (self . mz_specific_short) , mz_periods : FSet2 :: map_metazone_lookup (self . mz_periods) , decimal_formatter : self . decimal_formatter , _marker : PhantomData , } } }
};
}
