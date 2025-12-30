// Generated macro for DateDataMarkers (trait)
macro_rules! Depcrate_scaffold_fieldset_traitsDateDataMarkers {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"DateDataMarkers"}
// Dependencies: {}
# [doc = " A trait associating types for date formatting in any calendar"] # [doc = " (data markers only)."] # [doc = ""] # [doc = " This is a sealed trait implemented on field set markers."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] pub trait DateDataMarkers : UnstableSealed { # [doc = " Cross-calendar data markers for date skeleta."] type Skel : CalMarkers < ErasedPackedPatterns > ; # [doc = " Cross-calendar data markers for year names."] type Year : CalMarkers < YearNamesV1 > ; # [doc = " Cross-calendar data markers for month names."] type Month : CalMarkers < MonthNamesV1 > ; # [doc = " Marker for loading weekday names."] type WeekdayNamesV1 : DataMarker < DataStruct = LinearNames < 'static > > ; }
};
}
