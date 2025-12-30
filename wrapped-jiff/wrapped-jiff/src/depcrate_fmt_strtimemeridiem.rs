// Generated macro for Meridiem (enum)
macro_rules! Depcrate_fmt_strtimeMeridiem {
() => {
// Module: crate::fmt::strtime
// Provides: {"Meridiem"}
// Dependencies: {}
# [doc = " A label to disambiguate hours on a 12-hour clock."] # [doc = ""] # [doc = " This can be accessed on a [`BrokenDownTime`] via"] # [doc = " [`BrokenDownTime::meridiem`]."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum Meridiem { # [doc = " \"ante meridiem\" or \"before midday.\""] # [doc = ""] # [doc = " Specifically, this describes hours less than 12 on a 24-hour clock."] AM , # [doc = " \"post meridiem\" or \"after midday.\""] # [doc = ""] # [doc = " Specifically, this describes hours greater than 11 on a 24-hour clock."] PM , }
};
}
