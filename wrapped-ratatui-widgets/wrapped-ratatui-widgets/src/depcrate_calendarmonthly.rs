// Generated macro for Monthly (struct)
macro_rules! Depcrate_calendarMonthly {
() => {
// Module: crate::calendar
// Provides: {"Monthly"}
// Dependencies: {}
# [doc = " Display a month calendar for the month containing `display_date`"] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub struct Monthly < 'a , DS : DateStyler > { display_date : Date , events : DS , show_surrounding : Option < Style > , show_weekday : Option < Style > , show_month : Option < Style > , default_style : Style , block : Option < Block < 'a > > , }
};
}
