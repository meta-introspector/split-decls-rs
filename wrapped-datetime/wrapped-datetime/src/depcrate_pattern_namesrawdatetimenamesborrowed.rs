// Generated macro for RawDateTimeNamesBorrowed (struct)
macro_rules! Depcrate_pattern_namesRawDateTimeNamesBorrowed {
() => {
// Module: crate::pattern::names
// Provides: {"RawDateTimeNamesBorrowed"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] pub (crate) struct RawDateTimeNamesBorrowed < 'l > { year_names : OptionalNames < YearNameLength , & 'l YearNames < 'l > > , month_names : OptionalNames < MonthNameLength , & 'l MonthNames < 'l > > , weekday_names : OptionalNames < WeekdayNameLength , & 'l LinearNames < 'l > > , dayperiod_names : OptionalNames < DayPeriodNameLength , & 'l LinearNames < 'l > > , zone_essentials : OptionalNames < () , & 'l tz :: Essentials < 'l > > , locations_root : OptionalNames < () , & 'l tz :: Locations < 'l > > , locations : OptionalNames < () , & 'l tz :: Locations < 'l > > , exemplars_root : OptionalNames < () , & 'l tz :: ExemplarCities < 'l > > , exemplars : OptionalNames < () , & 'l tz :: ExemplarCities < 'l > > , mz_generic_long : OptionalNames < () , & 'l tz :: MzGeneric < 'l > > , mz_standard_long : OptionalNames < () , & 'l tz :: MzGeneric < 'l > > , mz_generic_short : OptionalNames < () , & 'l tz :: MzGeneric < 'l > > , mz_specific_long : OptionalNames < () , & 'l tz :: MzSpecific < 'l > > , mz_specific_short : OptionalNames < () , & 'l tz :: MzSpecific < 'l > > , mz_periods : OptionalNames < () , & 'l tz :: MzPeriod < 'l > > , pub (crate) decimal_formatter : Option < & 'l DecimalFormatter > , }
};
}
