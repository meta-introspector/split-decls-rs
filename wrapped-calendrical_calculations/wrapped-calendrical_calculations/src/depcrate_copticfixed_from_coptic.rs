// Generated macro for fixed_from_coptic (function)
macro_rules! Depcrate_copticfixed_from_coptic {
() => {
// Module: crate::coptic
// Provides: {"fixed_from_coptic"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1978>"] pub fn fixed_from_coptic (year : i32 , month : u8 , day : u8) -> RataDie { COPTIC_EPOCH - 1 + 365 * (year as i64 - 1) + year . div_euclid (4) as i64 + 30 * (month as i64 - 1) + day as i64 }
};
}
