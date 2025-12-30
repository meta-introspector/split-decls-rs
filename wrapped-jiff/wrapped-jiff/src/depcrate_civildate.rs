// Generated macro for date (function)
macro_rules! Depcrate_civildate {
() => {
// Module: crate::civil
// Provides: {"date"}
// Dependencies: {}
# [doc = " Creates a new `Date` value in a `const` context."] # [doc = ""] # [doc = " This is a convenience free function for [`Date::constant`]. It is intended"] # [doc = " to provide a terse syntax for constructing `Date` values from parameters"] # [doc = " that are known to be valid."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This routine panics when [`Date::new`] would return an error. That is,"] # [doc = " when the given year-month-day does not correspond to a valid date."] # [doc = " Namely, all of the following must be true:"] # [doc = ""] # [doc = " * The year must be in the range `-9999..=9999`."] # [doc = " * The month must be in the range `1..=12`."] # [doc = " * The day must be at least `1` and must be at most the number of days"] # [doc = " in the corresponding month. So for example, `2024-02-29` is valid but"] # [doc = " `2023-02-29` is not."] # [doc = ""] # [doc = " Similarly, when used in a const context, invalid parameters will prevent"] # [doc = " your Rust program from compiling."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::date;"] # [doc = ""] # [doc = " let d = date(2024, 2, 29);"] # [doc = " assert_eq!(d.year(), 2024);"] # [doc = " assert_eq!(d.month(), 2);"] # [doc = " assert_eq!(d.day(), 29);"] # [doc = " ```"] # [inline] pub const fn date (year : i16 , month : i8 , day : i8) -> Date { Date :: constant (year , month , day) }
};
}
