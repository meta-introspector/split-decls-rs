// Generated macro for DateWithDay (enum)
macro_rules! Depcrate_civil_dateDateWithDay {
() => {
// Module: crate::civil::date
// Provides: {"DateWithDay"}
// Dependencies: {}
# [doc = " Encodes the \"with day\" option of [`DateWith`]."] # [doc = ""] # [doc = " This encodes the invariant that `DateWith::day`, `DateWith::day_of_year`"] # [doc = " and `DateWith::day_of_year_no_leap` are all mutually exclusive and override"] # [doc = " each other."] # [doc = ""] # [doc = " Note that when \"day of year\" or \"day of year no leap\" are used, then if a"] # [doc = " day of month is set, it is ignored."] # [derive (Clone , Copy , Debug)] enum DateWithDay { OfMonth (i8) , OfYear (i16) , OfYearNoLeap (i16) , }
};
}
