// Generated macro for MonthInfo (struct)
macro_rules! Depcrate_typesMonthInfo {
() => {
// Module: crate::types
// Provides: {"MonthInfo"}
// Dependencies: {}
# [doc = " Representation of a formattable month."] # [derive (Copy , Clone , Debug , PartialEq)] # [non_exhaustive] pub struct MonthInfo { # [doc = " The ordinal month number in this given year. For calendars with leap months, all months after"] # [doc = " the leap month will end up with an incremented number."] # [doc = ""] # [doc = " In general, prefer using [`Month`]s in generic code."] pub ordinal : u8 , # [doc = " The [`Month`], used to distinguish months during leap years."] # [doc = ""] # [doc = " Round-trips through `Date` constructors like [`Date::try_new_from_codes`] and [`Date::try_from_fields`]."] # [doc = ""] # [doc = " This follows [Temporal's specification](https://tc39.es/proposal-intl-era-monthcode/#table-additional-month-codes)."] # [doc = " Months considered the \"same\" are equal: This means that the Hebrew months \"Adar\" and \"Adar II\" (\"Adar, but during a leap year\")"] # [doc = " are considered the same month, `Month::new(6)`."] # [doc = ""] # [doc = " [`Date::try_new_from_codes`]: crate::Date::try_new_from_codes"] # [doc = " [`Date::try_from_fields`]: crate::Date::try_from_fields"] pub value : Month , # [doc = " The [`Month::code()`] of [`Self::value`]."] # [deprecated (since = "2.2.0" , note = "use `value.code()")] pub standard_code : MonthCode , # [doc = " The [`Month::formatting_code()`] of [`Self::value`]."] # [deprecated (since = "2.2.0" , note = "use `value.formatting_code()")] pub formatting_code : MonthCode , }
};
}
