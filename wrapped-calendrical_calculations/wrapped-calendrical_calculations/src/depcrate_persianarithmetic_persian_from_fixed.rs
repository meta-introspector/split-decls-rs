// Generated macro for arithmetic_persian_from_fixed (function)
macro_rules! Depcrate_persianarithmetic_persian_from_fixed {
() => {
// Module: crate::persian
// Provides: {"arithmetic_persian_from_fixed"}
// Dependencies: {}
# [doc = " Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4857>"] # [doc = " Not used, but kept for comparative purposes"] pub fn arithmetic_persian_from_fixed (date : RataDie) -> Result < (i32 , u8 , u8) , I32CastError > { let year = arithmetic_persian_year_from_fixed (date) ; let year = i64_to_i32 (year) ? ; let day_of_year = 1_i64 + (date - fixed_from_arithmetic_persian (year , 1 , 1)) ; # [allow (unstable_name_collisions)] let month = if day_of_year <= 186 { day_of_year . div_ceil (31) as u8 } else { (day_of_year - 6) . div_ceil (30) as u8 } ; let day = (date - fixed_from_arithmetic_persian (year , month , 1) + 1) as u8 ; Ok ((year , month , day)) }
};
}
