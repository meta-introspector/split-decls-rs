// Generated macro for DateTimeWith (struct)
macro_rules! Depcrate_civil_datetimeDateTimeWith {
() => {
// Module: crate::civil::datetime
// Provides: {"DateTimeWith"}
// Dependencies: {}
# [doc = " A builder for setting the fields on a [`DateTime`]."] # [doc = ""] # [doc = " This builder is constructed via [`DateTime::with`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " The builder ensures one can chain together the individual components of a"] # [doc = " datetime without it failing at an intermediate step. For example, if you"] # [doc = " had a date of `2024-10-31T00:00:00` and wanted to change both the day and"] # [doc = " the month, and each setting was validated independent of the other, you"] # [doc = " would need to be careful to set the day first and then the month. In some"] # [doc = " cases, you would need to set the month first and then the day!"] # [doc = ""] # [doc = " But with the builder, you can set values in any order:"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::date;"] # [doc = ""] # [doc = " let dt1 = date(2024, 10, 31).at(0, 0, 0, 0);"] # [doc = " let dt2 = dt1.with().month(11).day(30).build()?;"] # [doc = " assert_eq!(dt2, date(2024, 11, 30).at(0, 0, 0, 0));"] # [doc = ""] # [doc = " let dt1 = date(2024, 4, 30).at(0, 0, 0, 0);"] # [doc = " let dt2 = dt1.with().day(31).month(7).build()?;"] # [doc = " assert_eq!(dt2, date(2024, 7, 31).at(0, 0, 0, 0));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] pub struct DateTimeWith { date_with : DateWith , time_with : TimeWith , }
};
}
