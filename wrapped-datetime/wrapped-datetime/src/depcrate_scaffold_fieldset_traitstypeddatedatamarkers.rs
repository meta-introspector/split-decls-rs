// Generated macro for TypedDateDataMarkers (trait)
macro_rules! Depcrate_scaffold_fieldset_traitsTypedDateDataMarkers {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"TypedDateDataMarkers"}
// Dependencies: {}
# [doc = " A trait associating types for date formatting in a specific calendar"] # [doc = " (data markers only)."] # [doc = ""] # [doc = " This is a sealed trait implemented on field set markers."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] pub trait TypedDateDataMarkers < C > : UnstableSealed { # [doc = " Marker for loading date skeleton patterns."] type DateSkeletonPatternsV1 : DataMarker < DataStruct = PackedPatterns < 'static > > ; # [doc = " Marker for loading year names."] type YearNamesV1 : DataMarker < DataStruct = YearNames < 'static > > ; # [doc = " Marker for loading month names."] type MonthNamesV1 : DataMarker < DataStruct = MonthNames < 'static > > ; # [doc = " Marker for loading weekday names."] type WeekdayNamesV1 : DataMarker < DataStruct = LinearNames < 'static > > ; }
};
}
