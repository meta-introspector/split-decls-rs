// Generated macro for is_leap_year (function)
macro_rules! Depcrate_dateis_leap_year {
() => {
// Module: crate::date
// Provides: {"is_leap_year"}
// Dependencies: {}
fn is_leap_year (y : u64) -> bool { y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) }
};
}
