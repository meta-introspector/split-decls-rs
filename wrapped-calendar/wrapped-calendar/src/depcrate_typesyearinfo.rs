// Generated macro for YearInfo (enum)
macro_rules! Depcrate_typesYearInfo {
() => {
// Module: crate::types
// Provides: {"YearInfo"}
// Dependencies: {}
# [doc = " The type of year: Calendars like Chinese don't have an era and instead format with cyclic years."] # [derive (Copy , Clone , Debug , PartialEq)] # [non_exhaustive] pub enum YearInfo { # [doc = " An era and a year in that era"] Era (EraYear) , # [doc = " A cyclic year, and the related ISO year"] # [doc = ""] # [doc = " Knowing the cyclic year is typically not enough to pinpoint a date, however cyclic calendars"] # [doc = " don't typically use eras, so disambiguation can be done by saying things like \"Year 甲辰 (2024)\""] Cyclic (CyclicYear) , }
};
}
