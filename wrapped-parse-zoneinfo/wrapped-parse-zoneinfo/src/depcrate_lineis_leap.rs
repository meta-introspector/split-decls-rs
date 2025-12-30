// Generated macro for is_leap (function)
macro_rules! Depcrate_lineis_leap {
() => {
// Module: crate::line
// Provides: {"is_leap"}
// Dependencies: {}
fn is_leap (year : i64) -> bool { year & 3 == 0 && (year % 25 != 0 || year & 15 == 0) }
};
}
