// Generated macro for impl_307 (impl)
macro_rules! Depcrate_pattern_namesimpl_307 {
() => {
// Module: crate::pattern::names
// Provides: {"impl_307"}
// Dependencies: {}
impl < FSet : DateTimeNamesMarker > DateTimeNames < FSet > { # [doc = " Maps a [`FixedCalendarDateTimeNames`] of a specific `FSet` to a more general `FSet`."] # [doc = ""] # [doc = " For example, this can transform a formatter for [`DateFieldSet`] to one for"] # [doc = " [`CompositeDateTimeFieldSet`]."] # [doc = ""] # [doc = " [`DateFieldSet`]: crate::fieldsets::enums::DateFieldSet"] # [doc = " [`CompositeDateTimeFieldSet`]: crate::fieldsets::enums::CompositeDateTimeFieldSet"] pub fn cast_into_fset < FSet2 : DateTimeNamesFrom < FSet > > (self) -> DateTimeNames < FSet2 > { DateTimeNames { inner : self . inner . cast_into_fset () , calendar : self . calendar , } } }
};
}
