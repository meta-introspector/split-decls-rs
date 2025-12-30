// Generated macro for set_weekday_with_number_from_monday (function)
macro_rules! Depcrate_format_parseset_weekday_with_number_from_monday {
() => {
// Module: crate::format::parse
// Provides: {"set_weekday_with_number_from_monday"}
// Dependencies: {}
fn set_weekday_with_number_from_monday (p : & mut Parsed , v : i64) -> ParseResult < () > { p . set_weekday (match v { 1 => Weekday :: Mon , 2 => Weekday :: Tue , 3 => Weekday :: Wed , 4 => Weekday :: Thu , 5 => Weekday :: Fri , 6 => Weekday :: Sat , 7 => Weekday :: Sun , _ => return Err (OUT_OF_RANGE) , }) }
};
}
