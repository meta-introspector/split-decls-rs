// Generated macro for DateWith (struct)
macro_rules! Depcrate_civil_dateDateWith {
() => {
// Module: crate::civil::date
// Provides: {"DateWith"}
// Dependencies: {}
# [doc = " A builder for setting the fields on a [`Date`]."] # [doc = ""] # [doc = " This builder is constructed via [`Date::with`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " The builder ensures one can chain together the individual components"] # [doc = " of a date without it failing at an intermediate step. For example,"] # [doc = " if you had a date of `2024-10-31` and wanted to change both the day"] # [doc = " and the month, and each setting was validated independent of the other,"] # [doc = " you would need to be careful to set the day first and then the month."] # [doc = " In some cases, you would need to set the month first and then the day!"] # [doc = ""] # [doc = " But with the builder, you can set values in any order:"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::date;"] # [doc = ""] # [doc = " let d1 = date(2024, 10, 31);"] # [doc = " let d2 = d1.with().month(11).day(30).build()?;"] # [doc = " assert_eq!(d2, date(2024, 11, 30));"] # [doc = ""] # [doc = " let d1 = date(2024, 4, 30);"] # [doc = " let d2 = d1.with().day(31).month(7).build()?;"] # [doc = " assert_eq!(d2, date(2024, 7, 31));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct DateWith { original : Date , year : Option < DateWithYear > , month : Option < i8 > , day : Option < DateWithDay > , }
};
}
