// Generated macro for is_leap_year (function)
macro_rules! Depcrate_copticis_leap_year {
() => {
// Module: crate::coptic
// Provides: {"is_leap_year"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1973>"] # [inline (always)] pub const fn is_leap_year (year : i32) -> bool { (year + 1) % 4 == 0 }
};
}
