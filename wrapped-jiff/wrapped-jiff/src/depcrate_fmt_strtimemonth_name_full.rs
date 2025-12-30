// Generated macro for month_name_full (function)
macro_rules! Depcrate_fmt_strtimemonth_name_full {
() => {
// Module: crate::fmt::strtime
// Provides: {"month_name_full"}
// Dependencies: {}
# [doc = " Returns the \"full\" month name."] fn month_name_full (month : t :: Month) -> & 'static str { match month . get () { 1 => "January" , 2 => "February" , 3 => "March" , 4 => "April" , 5 => "May" , 6 => "June" , 7 => "July" , 8 => "August" , 9 => "September" , 10 => "October" , 11 => "November" , 12 => "December" , unk => unreachable ! ("invalid month {unk}") , } }
};
}
