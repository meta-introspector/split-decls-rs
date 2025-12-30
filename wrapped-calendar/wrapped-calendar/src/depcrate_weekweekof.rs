// Generated macro for WeekOf (struct)
macro_rules! Depcrate_weekWeekOf {
() => {
// Module: crate::week
// Provides: {"WeekOf"}
// Dependencies: {}
# [doc = " The week number assigned to a given week according to a calendar."] # [derive (Debug , PartialEq)] # [allow (clippy :: exhaustive_structs)] pub (crate) struct WeekOf { # [doc = " Week of month/year. 1 based."] pub week : u8 , # [doc = " The month/year that this week is in, relative to the month/year of the input date."] pub unit : RelativeUnit , }
};
}
