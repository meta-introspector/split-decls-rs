// Generated macro for impl_430 (impl)
macro_rules! Depcrate_weekimpl_430 {
() => {
// Module: crate::week
// Provides: {"impl_430"}
// Dependencies: {}
impl WeekdaySetIterator { # [doc = " Creates the Iterator. Sets `current_day` to the day after `first_weekday`."] pub (crate) fn new (first_weekday : Weekday , weekend : WeekdaySet) -> Self { WeekdaySetIterator { first_weekday , current_day : first_weekday , weekend , } } }
};
}
