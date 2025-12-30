// Generated macro for weekday_name_abbrev (function)
macro_rules! Depcrate_fmt_strtimeweekday_name_abbrev {
() => {
// Module: crate::fmt::strtime
// Provides: {"weekday_name_abbrev"}
// Dependencies: {}
# [doc = " Returns an abbreviated weekday name."] fn weekday_name_abbrev (wd : Weekday) -> & 'static str { match wd { Weekday :: Sunday => "Sun" , Weekday :: Monday => "Mon" , Weekday :: Tuesday => "Tue" , Weekday :: Wednesday => "Wed" , Weekday :: Thursday => "Thu" , Weekday :: Friday => "Fri" , Weekday :: Saturday => "Sat" , } }
};
}
