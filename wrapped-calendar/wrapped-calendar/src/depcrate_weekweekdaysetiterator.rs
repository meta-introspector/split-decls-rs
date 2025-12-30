// Generated macro for WeekdaySetIterator (struct)
macro_rules! Depcrate_weekWeekdaySetIterator {
() => {
// Module: crate::week
// Provides: {"WeekdaySetIterator"}
// Dependencies: {}
# [doc = " [Iterator] that yields weekdays that are part of the weekend."] # [derive (Clone , Copy , Debug , PartialEq)] pub struct WeekdaySetIterator { # [doc = " Determines the order in which we should start reading values from `weekend`."] first_weekday : Weekday , # [doc = " Day being evaluated."] current_day : Weekday , # [doc = " Bitset to read weekdays from."] weekend : WeekdaySet , }
};
}
