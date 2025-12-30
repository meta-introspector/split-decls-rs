// Generated macro for DaySpec (enum)
macro_rules! Depcrate_lineDaySpec {
() => {
// Module: crate::line
// Provides: {"DaySpec"}
// Dependencies: {}
# [doc = " A **day** definition field."] # [doc = ""] # [doc = " This can be given in either absolute terms (such as “the fifth day of the"] # [doc = " month”), or relative terms (such as “the last Sunday of the month”, or"] # [doc = " “the last Friday before or including the 13th”)."] # [doc = ""] # [doc = " Note that in the last example, it’s allowed for that particular Friday to"] # [doc = " *be* the 13th in question."] # [derive (PartialEq , Debug , Copy , Clone)] pub enum DaySpec { # [doc = " A specific day of the month, given by its number."] Ordinal (i8) , # [doc = " The last day of the month with a specific weekday."] Last (Weekday) , # [doc = " The **last** day with the given weekday **before** (or including) a"] # [doc = " day with a specific number."] LastOnOrBefore (Weekday , i8) , # [doc = " The **first** day with the given weekday **after** (or including) a"] # [doc = " day with a specific number."] FirstOnOrAfter (Weekday , i8) , }
};
}
