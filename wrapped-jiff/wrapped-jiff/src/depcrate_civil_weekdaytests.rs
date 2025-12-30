// Generated macro for tests (module)
macro_rules! Depcrate_civil_weekdaytests {
() => {
// Module: crate::civil::weekday
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; quickcheck :: quickcheck ! { fn prop_since_add_equals_self (wd1 : Weekday , wd2 : Weekday) -> bool { let days = wd1 . since (wd2) ; wd2 . wrapping_add (days) == wd1 } fn prop_until_add_equals_other (wd1 : Weekday , wd2 : Weekday) -> bool { let days = wd1 . until (wd2) ; wd1 . wrapping_add (days) == wd2 } } }
};
}
