// Generated macro for AllFixedCalendarPatternDataMarkers (trait)
macro_rules! Depcrate_scaffold_fieldset_traitsAllFixedCalendarPatternDataMarkers {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"AllFixedCalendarPatternDataMarkers"}
// Dependencies: {}
# [doc = " Trait to consolidate data provider markers defined by this crate"] # [doc = " for datetime skeleton patterns with a fixed calendar."] # [doc = ""] # [doc = " This trait is implemented on all providers that support datetime skeleton patterns,"] # [doc = " including [`crate::provider::Baked`]."] # [rustfmt :: skip] pub trait AllFixedCalendarPatternDataMarkers < C : CldrCalendar , FSet : DateTimeMarkers > : DataProvider < < FSet :: D as TypedDateDataMarkers < C > > :: DateSkeletonPatternsV1 > + DataProvider < < FSet :: T as TimeMarkers > :: TimeSkeletonPatternsV1 > + DataProvider < FSet :: GluePatternV1 > where FSet :: D : TypedDateDataMarkers < C > , FSet :: T : TimeMarkers , FSet :: Z : ZoneMarkers , { }
};
}
