// Generated macro for impl_1038 (impl)
macro_rules! Depcrate_scaffold_fieldset_traitsimpl_1038 {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"impl_1038"}
// Dependencies: {}
# [rustfmt :: skip] impl < T , C , FSet > AllFixedCalendarPatternDataMarkers < C , FSet > for T where C : CldrCalendar , FSet : DateTimeMarkers , FSet :: D : TypedDateDataMarkers < C > , FSet :: T : TimeMarkers , FSet :: Z : ZoneMarkers , T : ? Sized + DataProvider < < FSet :: D as TypedDateDataMarkers < C > > :: DateSkeletonPatternsV1 > + DataProvider < < FSet :: T as TimeMarkers > :: TimeSkeletonPatternsV1 > + DataProvider < FSet :: GluePatternV1 > , { }
};
}
