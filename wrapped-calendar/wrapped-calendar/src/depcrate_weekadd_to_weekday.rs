// Generated macro for add_to_weekday (function)
macro_rules! Depcrate_weekadd_to_weekday {
() => {
// Module: crate::week
// Provides: {"add_to_weekday"}
// Dependencies: {}
# [doc = " Returns the weekday that's `num_days` after `weekday`."] fn add_to_weekday (weekday : Weekday , num_days : i32) -> Weekday { let new_weekday = (7 + (weekday as i32) + (num_days % 7)) % 7 ; Weekday :: from_days_since_sunday (new_weekday as isize) }
};
}
