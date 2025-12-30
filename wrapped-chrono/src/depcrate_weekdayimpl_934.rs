// Generated macro for impl_934 (impl)
macro_rules! Depcrate_weekdayimpl_934 {
() => {
// Module: crate::weekday
// Provides: {"impl_934"}
// Dependencies: {}
# [doc = " Any weekday can be represented as an integer from 0 to 6, which equals to"] # [doc = " [`Weekday::num_days_from_monday`](#method.num_days_from_monday) in this implementation."] # [doc = " Do not heavily depend on this though; use explicit methods whenever possible."] impl TryFrom < u8 > for Weekday { type Error = OutOfRange ; fn try_from (value : u8) -> Result < Self , Self :: Error > { match value { 0 => Ok (Weekday :: Mon) , 1 => Ok (Weekday :: Tue) , 2 => Ok (Weekday :: Wed) , 3 => Ok (Weekday :: Thu) , 4 => Ok (Weekday :: Fri) , 5 => Ok (Weekday :: Sat) , 6 => Ok (Weekday :: Sun) , _ => Err (OutOfRange :: new ()) , } } }
};
}
