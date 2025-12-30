// Generated macro for set_weekday_with_num_days_from_sunday (function)
macro_rules! Depcrate_format_parseset_weekday_with_num_days_from_sunday {
() => {
// Module: crate::format::parse
// Provides: {"set_weekday_with_num_days_from_sunday"}
// Dependencies: {}
fn set_weekday_with_num_days_from_sunday (p : & mut Parsed , v : i64) -> ParseResult < () > { p . set_weekday (match v { 0 => Weekday :: Sun , 1 => Weekday :: Mon , 2 => Weekday :: Tue , 3 => Weekday :: Wed , 4 => Weekday :: Thu , 5 => Weekday :: Fri , 6 => Weekday :: Sat , _ => return Err (OUT_OF_RANGE) , }) }
};
}
