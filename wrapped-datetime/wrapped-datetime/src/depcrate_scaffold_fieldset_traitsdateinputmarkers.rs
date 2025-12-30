// Generated macro for DateInputMarkers (trait)
macro_rules! Depcrate_scaffold_fieldset_traitsDateInputMarkers {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"DateInputMarkers"}
// Dependencies: {}
# [doc = " A trait associating types for date formatting in any calendar"] # [doc = " (input types only)."] # [doc = ""] # [doc = " This is a sealed trait implemented on field set markers."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] pub trait DateInputMarkers : UnstableSealed { # [doc = " Marker for resolving the year input field."] type YearInput : IntoOption < YearInfo > ; # [doc = " Marker for resolving the month input field."] type MonthInput : IntoOption < MonthInfo > ; # [doc = " Marker for resolving the day-of-month input field."] type DayOfMonthInput : IntoOption < DayOfMonth > ; # [doc = " Marker for resolving the day-of-year input field."] type DayOfYearInput : IntoOption < DayOfYear > ; # [doc = " Marker for resolving the day-of-year input field."] type RataDieInput : IntoOption < RataDie > ; # [doc = " Marker for resolving the day-of-week input field."] type DayOfWeekInput : IntoOption < Weekday > ; }
};
}
