// Generated macro for month_name_abbrev (function)
macro_rules! Depcrate_fmt_strtimemonth_name_abbrev {
() => {
// Module: crate::fmt::strtime
// Provides: {"month_name_abbrev"}
// Dependencies: {}
# [doc = " Returns the abbreviated month name."] fn month_name_abbrev (month : t :: Month) -> & 'static str { match month . get () { 1 => "Jan" , 2 => "Feb" , 3 => "Mar" , 4 => "Apr" , 5 => "May" , 6 => "Jun" , 7 => "Jul" , 8 => "Aug" , 9 => "Sep" , 10 => "Oct" , 11 => "Nov" , 12 => "Dec" , unk => unreachable ! ("invalid month {unk}") , } }
};
}
