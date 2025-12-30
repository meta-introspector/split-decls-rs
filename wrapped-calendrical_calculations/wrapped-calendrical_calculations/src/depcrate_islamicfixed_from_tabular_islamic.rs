// Generated macro for fixed_from_tabular_islamic (function)
macro_rules! Depcrate_islamicfixed_from_tabular_islamic {
() => {
// Module: crate::islamic
// Provides: {"fixed_from_tabular_islamic"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2076>"] pub fn fixed_from_tabular_islamic (year : i32 , month : u8 , day : u8 , epoch : RataDie) -> RataDie { let year = i64 :: from (year) ; let month = i64 :: from (month) ; let day = i64 :: from (day) ; RataDie :: new ((epoch . to_i64_date () - 1) + (year - 1) * 354 + (3 + year * 11) . div_euclid (30) + 29 * (month - 1) + month . div_euclid (2) + day ,) }
};
}
