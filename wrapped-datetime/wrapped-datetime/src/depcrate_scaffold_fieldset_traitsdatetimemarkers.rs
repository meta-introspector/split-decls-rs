// Generated macro for DateTimeMarkers (trait)
macro_rules! Depcrate_scaffold_fieldset_traitsDateTimeMarkers {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"DateTimeMarkers"}
// Dependencies: {}
# [doc = " A trait associating constants and types implementing various other traits"] # [doc = " required for datetime formatting."] # [doc = ""] # [doc = " This is a sealed trait implemented on field set markers."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] pub trait DateTimeMarkers : UnstableSealed + DateTimeNamesMarker { # [doc = " Associated types for date formatting."] # [doc = ""] # [doc = " Should implement [`DateDataMarkers`], [`TypedDateDataMarkers`], and [`DateInputMarkers`]."] type D ; # [doc = " Associated types for time formatting."] # [doc = ""] # [doc = " Should implement [`TimeMarkers`]."] type T ; # [doc = " Associated types for time zone formatting."] # [doc = ""] # [doc = " Should implement [`ZoneMarkers`]."] type Z ; # [doc = " Marker for loading the date/time glue pattern."] type GluePatternV1 : DataMarker < DataStruct = GluePattern < 'static > > ; }
};
}
