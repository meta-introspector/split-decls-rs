// Generated macro for impl_1040 (impl)
macro_rules! Depcrate_scaffold_fieldset_traitsimpl_1040 {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"impl_1040"}
// Dependencies: {}
# [rustfmt :: skip] impl < T , C , FSet > AllFixedCalendarFormattingDataMarkers < C , FSet > for T where C : CldrCalendar , FSet : DateTimeMarkers , FSet :: D : TypedDateDataMarkers < C > , FSet :: T : TimeMarkers , FSet :: Z : ZoneMarkers , T : ? Sized + DataProvider < < FSet :: D as TypedDateDataMarkers < C > > :: YearNamesV1 > + DataProvider < < FSet :: D as TypedDateDataMarkers < C > > :: MonthNamesV1 > + DataProvider < < FSet :: D as TypedDateDataMarkers < C > > :: DateSkeletonPatternsV1 > + DataProvider < < FSet :: D as TypedDateDataMarkers < C > > :: WeekdayNamesV1 > + DataProvider < < FSet :: T as TimeMarkers > :: DayPeriodNamesV1 > + DataProvider < < FSet :: T as TimeMarkers > :: TimeSkeletonPatternsV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: EssentialsV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: LocationsV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: LocationsRootV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: ExemplarCitiesV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: ExemplarCitiesRootV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: GenericLongV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: GenericShortV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: StandardLongV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: SpecificLongV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: SpecificShortV1 > + DataProvider < < FSet :: Z as ZoneMarkers > :: MetazonePeriodV1 > + DataProvider < FSet :: GluePatternV1 > + AllFixedCalendarPatternDataMarkers < C , FSet > { }
};
}
