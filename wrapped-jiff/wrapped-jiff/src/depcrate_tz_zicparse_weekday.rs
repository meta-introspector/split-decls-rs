// Generated macro for parse_weekday (function)
macro_rules! Depcrate_tz_zicparse_weekday {
() => {
// Module: crate::tz::zic
// Provides: {"parse_weekday"}
// Dependencies: {}
# [doc = " Parses a possibly abbreviated weekday from the given string."] fn parse_weekday (string : & str) -> Result < Weekday , Error > { static WEEKDAY_PREFIXES : & [(Weekday , & str , & str)] = & [(Weekday :: Monday , "Monday" , "M") , (Weekday :: Tuesday , "Tuesday" , "Tu") , (Weekday :: Wednesday , "Wednesday" , "W") , (Weekday :: Thursday , "Thursday" , "Th") , (Weekday :: Friday , "Friday" , "F") , (Weekday :: Saturday , "Saturday" , "Sa") , (Weekday :: Sunday , "Sunday" , "Su") ,] ; for & (weekday , name , prefix) in WEEKDAY_PREFIXES { if string . starts_with (prefix) && name . starts_with (string) { return Ok (weekday) ; } } Err (err ! ("unrecognized day of the week: {string:?}")) }
};
}
