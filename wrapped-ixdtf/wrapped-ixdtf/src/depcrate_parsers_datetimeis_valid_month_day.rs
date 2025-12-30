// Generated macro for is_valid_month_day (function)
macro_rules! Depcrate_parsers_datetimeis_valid_month_day {
() => {
// Module: crate::parsers::datetime
// Provides: {"is_valid_month_day"}
// Dependencies: {}
fn is_valid_month_day (month : u8 , day : u8) -> bool { match month { 2 | 4 | 6 | 9 | 11 if day >= 31 => false , 2 if day == 30 => false , _ => day <= 31 , } }
};
}
