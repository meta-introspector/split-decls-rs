// Generated macro for time (function)
macro_rules! Depcrate_civiltime {
() => {
// Module: crate::civil
// Provides: {"time"}
// Dependencies: {}
# [doc = " Creates a new `Time` value in a `const` context."] # [doc = ""] # [doc = " This is a convenience free function for [`Time::constant`]. It is intended"] # [doc = " to provide a terse syntax for constructing `Time` values from parameters"] # [doc = " that are known to be valid."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics if the given values do not correspond to a valid `Time`."] # [doc = " All of the following conditions must be true:"] # [doc = ""] # [doc = " * `0 <= hour <= 23`"] # [doc = " * `0 <= minute <= 59`"] # [doc = " * `0 <= second <= 59`"] # [doc = " * `0 <= subsec_nanosecond <= 999,999,999`"] # [doc = ""] # [doc = " Similarly, when used in a const context, invalid parameters will"] # [doc = " prevent your Rust program from compiling."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows an example of a valid time in a `const` context:"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::Time;"] # [doc = ""] # [doc = " const BEDTIME: Time = Time::constant(21, 30, 5, 123_456_789);"] # [doc = " assert_eq!(BEDTIME.hour(), 21);"] # [doc = " assert_eq!(BEDTIME.minute(), 30);"] # [doc = " assert_eq!(BEDTIME.second(), 5);"] # [doc = " assert_eq!(BEDTIME.millisecond(), 123);"] # [doc = " assert_eq!(BEDTIME.microsecond(), 456);"] # [doc = " assert_eq!(BEDTIME.nanosecond(), 789);"] # [doc = " assert_eq!(BEDTIME.subsec_nanosecond(), 123_456_789);"] # [doc = " ```"] # [inline] pub const fn time (hour : i8 , minute : i8 , second : i8 , subsec_nanosecond : i32 ,) -> Time { Time :: constant (hour , minute , second , subsec_nanosecond) }
};
}
