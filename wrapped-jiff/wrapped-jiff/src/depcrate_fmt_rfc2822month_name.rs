// Generated macro for month_name (function)
macro_rules! Depcrate_fmt_rfc2822month_name {
() => {
// Module: crate::fmt::rfc2822
// Provides: {"month_name"}
// Dependencies: {}
fn month_name (month : i8) -> & 'static str { match month { 1 => "Jan" , 2 => "Feb" , 3 => "Mar" , 4 => "Apr" , 5 => "May" , 6 => "Jun" , 7 => "Jul" , 8 => "Aug" , 9 => "Sep" , 10 => "Oct" , 11 => "Nov" , 12 => "Dec" , _ => unreachable ! ("invalid month value {month}") , } }
};
}
