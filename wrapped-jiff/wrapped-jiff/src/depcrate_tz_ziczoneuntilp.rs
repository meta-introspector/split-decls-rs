// Generated macro for ZoneUntilP (enum)
macro_rules! Depcrate_tz_zicZoneUntilP {
() => {
// Module: crate::tz::zic
// Provides: {"ZoneUntilP"}
// Dependencies: {}
# [doc = " The time until a particular zone is active."] # [doc = ""] # [doc = " This time is treated as an exclusive boundary. That is, times at precisely"] # [doc = " this point are not governed by its corresponding zone."] # [derive (Clone , Debug , Eq , PartialEq)] enum ZoneUntilP { Year { year : t :: Year , } , YearMonth { year : t :: Year , month : RuleInP , } , YearMonthDay { year : t :: Year , month : RuleInP , day : RuleOnP , } , YearMonthDayTime { year : t :: Year , month : RuleInP , day : RuleOnP , # [doc = " Note that adding a span to the year/month/day could overflow"] # [doc = " the allowed maximum time. This isn't handled until this type is"] # [doc = " converted into higher level data types."] duration : RuleAtP , } , }
};
}
