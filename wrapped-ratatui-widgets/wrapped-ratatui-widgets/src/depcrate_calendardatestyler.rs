// Generated macro for DateStyler (trait)
macro_rules! Depcrate_calendarDateStyler {
() => {
// Module: crate::calendar
// Provides: {"DateStyler"}
// Dependencies: {}
# [doc = " Provides a method for styling a given date. [Monthly] is generic on this trait, so any type"] # [doc = " that implements this trait can be used."] pub trait DateStyler { # [doc = " Given a date, return a style for that date"] fn get_style (& self , date : Date) -> Style ; }
};
}
