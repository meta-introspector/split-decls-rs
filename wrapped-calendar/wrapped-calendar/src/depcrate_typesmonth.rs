// Generated macro for Month (struct)
macro_rules! Depcrate_typesMonth {
() => {
// Module: crate::types
// Provides: {"Month"}
// Dependencies: {}
# [doc = " Representation of a month in a year"] # [doc = ""] # [doc = " A month has a \"number\" and \"leap flag\". In calendars without leap months (non-lunisolar"] # [doc = " calendars), the month with number n is always the nth month of the year (_ordinal month_),"] # [doc = " for example the Gregorian September is `Month:new(9)` and the 9th month of the year."] # [doc = " However, in calendars with leap months (lunisolar calendars), such as the Hebrew calendar,"] # [doc = " a month might repeat (leap) without affecting the number of each subsequent month (but"] # [doc = " obviously affecting their _ordinal number_). For example, the Hebrew month Nisan"] # [doc = " (`Month::new(7)`) might be the 7th or 8th month of the year, depending if the month"] # [doc = " Adar was repeated or not."] # [doc = ""] # [doc = " Check the docs for a particular calendar for details on what its months are."] # [doc = ""] # [doc = " This concept of months matches the \"month code\" in [Temporal], and borrows its string"] # [doc = " representation:"] # [doc = " * `Month::new(7)` = `M07`"] # [doc = " * `Month::leap(2)` = `M02L`"] # [doc = ""] # [doc = " [Temporal]: https://tc39.es/proposal-intl-era-monthcode/"] # [derive (Copy , Clone , Debug , PartialEq , Hash , Eq)] pub struct Month { # [doc = " Month number between 0 and 99"] number : u8 , leap_status : LeapStatus , }
};
}
