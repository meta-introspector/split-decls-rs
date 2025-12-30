// Generated macro for RelativeWeek (enum)
macro_rules! Depcrate_weekRelativeWeek {
() => {
// Module: crate::week
// Provides: {"RelativeWeek"}
// Dependencies: {}
# [doc = " Which year or month that a calendar assigns a week to relative to the year/month"] # [doc = " the week is in."] # [derive (Clone , Copy , Debug , PartialEq)] # [expect (clippy :: enum_variant_names)] enum RelativeWeek { # [doc = " A week that is assigned to the last week of the previous year/month. e.g. 2021-01-01 is week 54 of 2020 per the ISO calendar."] LastWeekOfPreviousUnit , # [doc = " A week that's assigned to the current year/month. The offset is 1-based. e.g. 2021-01-11 is week 2 of 2021 per the ISO calendar so would be WeekOfCurrentUnit(2)."] WeekOfCurrentUnit (u8) , # [doc = " A week that is assigned to the first week of the next year/month. e.g. 2019-12-31 is week 1 of 2020 per the ISO calendar."] FirstWeekOfNextUnit , }
};
}
