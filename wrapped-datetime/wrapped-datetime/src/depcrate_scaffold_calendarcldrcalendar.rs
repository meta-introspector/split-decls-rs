// Generated macro for CldrCalendar (trait)
macro_rules! Depcrate_scaffold_calendarCldrCalendar {
() => {
// Module: crate::scaffold::calendar
// Provides: {"CldrCalendar"}
// Dependencies: {}
# [doc = " A calendar that can be found in CLDR."] # [doc = ""] # [doc = " New implementors of this trait will likely also wish to modify `get_era_code_map()`"] # [doc = " in the CLDR transformer to support any new era maps."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this"] # [doc = " trait, please consider using a type from the implementors listed below."] # [doc = " </div>"] pub trait CldrCalendar : UnstableSealed { # [doc = " The data marker for loading year symbols for this calendar."] type YearNamesV1 : DataMarker < DataStruct = YearNames < 'static > > ; # [doc = " The data marker for loading month symbols for this calendar."] type MonthNamesV1 : DataMarker < DataStruct = MonthNames < 'static > > ; # [doc = " The data marker for loading skeleton patterns for this calendar."] type SkeletaV1 : DataMarker < DataStruct = PackedPatterns < 'static > > ; }
};
}
