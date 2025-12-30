// Generated macro for TimeMarkers (trait)
macro_rules! Depcrate_scaffold_fieldset_traitsTimeMarkers {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"TimeMarkers"}
// Dependencies: {}
# [doc = " A trait associating types for time formatting"] # [doc = " (input types and data markers)."] # [doc = ""] # [doc = " This is a sealed trait implemented on field set markers."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] pub trait TimeMarkers : UnstableSealed { # [doc = " Marker for resolving the day-of-month input field."] type HourInput : IntoOption < Hour > ; # [doc = " Marker for resolving the day-of-week input field."] type MinuteInput : IntoOption < Minute > ; # [doc = " Marker for resolving the day-of-year input field."] type SecondInput : IntoOption < Second > ; # [doc = " Marker for resolving the any-calendar-kind input field."] type NanosecondInput : IntoOption < Nanosecond > ; # [doc = " Marker for loading time skeleton patterns."] type TimeSkeletonPatternsV1 : DataMarker < DataStruct = PackedPatterns < 'static > > ; # [doc = " Marker for loading day period names."] type DayPeriodNamesV1 : DataMarker < DataStruct = LinearNames < 'static > > ; }
};
}
