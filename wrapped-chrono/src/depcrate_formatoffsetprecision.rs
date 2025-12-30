// Generated macro for OffsetPrecision (enum)
macro_rules! Depcrate_formatOffsetPrecision {
() => {
// Module: crate::format
// Provides: {"OffsetPrecision"}
// Dependencies: {}
# [doc = " The precision of an offset from UTC formatting item."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum OffsetPrecision { # [doc = " Format offset from UTC as only hours. Not recommended, it is not uncommon for timezones to"] # [doc = " have an offset of 30 minutes, 15 minutes, etc."] # [doc = " Any minutes and seconds get truncated."] Hours , # [doc = " Format offset from UTC as hours and minutes."] # [doc = " Any seconds will be rounded to the nearest minute."] Minutes , # [doc = " Format offset from UTC as hours, minutes and seconds."] Seconds , # [doc = " Format offset from UTC as hours, and optionally with minutes."] # [doc = " Any seconds will be rounded to the nearest minute."] OptionalMinutes , # [doc = " Format offset from UTC as hours and minutes, and optionally seconds."] OptionalSeconds , # [doc = " Format offset from UTC as hours and optionally minutes and seconds."] OptionalMinutesAndSeconds , }
};
}
