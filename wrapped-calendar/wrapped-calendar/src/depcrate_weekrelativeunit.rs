// Generated macro for RelativeUnit (enum)
macro_rules! Depcrate_weekRelativeUnit {
() => {
// Module: crate::week
// Provides: {"RelativeUnit"}
// Dependencies: {}
# [doc = " The year or month that a calendar assigns a week to relative to the year/month that it is in."] # [derive (Debug , PartialEq)] # [allow (clippy :: exhaustive_enums)] pub (crate) enum RelativeUnit { # [doc = " A week that is assigned to previous year/month. e.g. 2021-01-01 is week 54 of 2020 per the ISO calendar."] Previous , # [doc = " A week that's assigned to the current year/month. e.g. 2021-01-11 is week 2 of 2021 per the ISO calendar."] Current , # [doc = " A week that is assigned to the next year/month. e.g. 2019-12-31 is week 1 of 2020 per the ISO calendar."] Next , }
};
}
